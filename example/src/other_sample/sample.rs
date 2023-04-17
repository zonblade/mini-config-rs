use crate::config::{SampleConfig,OtherSampleConfig};

pub fn test_get_data(){
    println!("DatabaseName: {:?}", SampleConfig::DatabaseName.val());
    println!("BrokerName: {:?}", SampleConfig::BrokerName.val());
    println!("OtherSettings: {:?}", OtherSampleConfig::OtherSettings.val());
}