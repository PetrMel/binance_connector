pub mod price_levels_engine {
    use serde::ser::{SerializeSeq, SerializeStruct};

    use crate::json_helper::PriceLevelsSnapshot;

#[derive(Debug)]
struct Bids {
    levels: std::collections::BTreeMap<String, String>
}

#[derive(Debug)]
struct Asks {
    levels: std::collections::BTreeMap<String, String>
}

//#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct PriceLevels {
    last_update_id: i64,
    bids: Bids,
    asks: Asks
}

impl PriceLevels {

    pub fn make_init_price_levels_from_snapshot(snapshot: PriceLevelsSnapshot) -> Self {
        let bids_local = std::collections::BTreeMap::from_iter(snapshot.bids);
        let asks_local = std::collections::BTreeMap::from_iter(snapshot.asks);
        
        Self { last_update_id: snapshot.lastUpdateId, bids: Bids { levels: bids_local }, asks: Asks { levels: asks_local }}
    }

    fn update_one_side_from_vec(side: &mut std::collections::BTreeMap<String, String>, from: &Vec<(String, String)>) {
        for elem in from {
            let parsed: f32 = elem.1.parse().unwrap();
            //TODO better to compare double with 0 using precision from https://www.binance.com/en/trade/BTC_USDT?type=spot
            if parsed == 0.0 {
                side.remove(elem.0.to_string().as_str());
                continue;
            }

            side.insert(elem.0.clone(), elem.1.clone());
        }
    }

    pub fn update_from_incremental (&mut self, inc_upd: crate::json_helper::PriceLevelsIncremental, conn_num: i8) -> Result<(), eyre::Error> {
        //for check println!("{conn_num:?} : {inc_upd:?}");
        if inc_upd.u < self.last_update_id {
            // Nothing to do, return to skip
            return Ok(());
        }
 
        //for check println!("in: {conn_num:?} : {inc_upd:?}");

        if inc_upd.U > self.last_update_id+1 {
            return Err(eyre::eyre!("Something went wrong"));
        }

        self.last_update_id = inc_upd.u;
        
        Self::update_one_side_from_vec(&mut self.bids.levels, &inc_upd.b);
        Self::update_one_side_from_vec(&mut self.asks.levels, &inc_upd.a);

        Ok(())
    }

    pub fn as_json_text(&mut self) -> String {
        let json_text = serde_json::to_string(self).unwrap();
        return json_text;
    }
    


}

impl serde::Serialize for PriceLevels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PriceLevels", 3)?;
        state.serialize_field( "lastUpdateId", &self.last_update_id)?;
        state.serialize_field("bids", &self.bids)?;
        state.serialize_field("asks", &self.asks)?;
        state.end()
    }
}

impl serde::Serialize for Bids
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let size = usize::max(self.levels.len(), 100);
        let mut seq = serializer.serialize_seq(Some(size))?;
        let rev_iter = self.levels.iter().rev().enumerate();
        for elem in  rev_iter {
            seq.serialize_element(&elem.1)?;
            if elem.0 == 100-1 {
                break;
            }
        }
        seq.end()
    }
}

impl serde::Serialize for Asks
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let size = usize::max(self.levels.len(), 100);
        let mut seq = serializer.serialize_seq(Some(size))?;
        let iter = self.levels.iter().enumerate();
        for elem in  iter {
            seq.serialize_element(&elem.1)?;
            if elem.0 == 100-1 {
                break;
            }
        }
        seq.end()
    }
}

}