pub mod price_levels_engine {
    

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct PriceLevels {
    pub last_update_id: i64,
    pub bids: std::collections::BTreeMap<String, String>,
    pub asks: std::collections::BTreeMap<String, String>
}

impl PriceLevels {

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

    pub fn update_from_incremental (&mut self, inc_upd: crate::json_helper::PriceLevelsIncremental, conn_num: i8) {
        println!("{conn_num:?} : {inc_upd:?}");
        if inc_upd.u < self.last_update_id {
            // Nothing to do
            return;
        }
 
        println!("in: {conn_num:?} : {inc_upd:?}");

        if inc_upd.U > self.last_update_id+1 {
            panic!("Something went wrong");
        }

        self.last_update_id = inc_upd.u;
        
        //TODO make closure
        Self::update_one_side_from_vec(&mut self.asks, &inc_upd.a);
        Self::update_one_side_from_vec(&mut self.bids, &inc_upd.b);
    }

    pub fn as_json_text(&mut self) -> String {
        //TODO make only slice of 100 for bids and asks in here
        let json_text = serde_json::to_string(self).unwrap();
        return json_text;
    }

}

}