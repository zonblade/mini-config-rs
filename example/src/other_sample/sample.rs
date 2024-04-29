use crate::config::{SampleConfig,OtherSampleConfig};

pub fn test_get_data(){
    let text = SampleConfig::DatabaseName.get_str();
    let text2 = SampleConfig::DatabaseName.get();
    println!("DatabaseName: {:?}, {:?}", text, text2);
    println!("BrokerName: {:?}", SampleConfig::BrokerName.val());
    println!("OtherSettings: {:?}", OtherSampleConfig::OtherSettings.val());
}