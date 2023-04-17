use mini_config::Configure;

///
/// enum must have "Debug" and "Clone"
/// oterwise it will throw an error
/// and cannot be compiled
/// 
#[derive(Debug, Clone, Configure)]
pub enum SampleConfig {
    DatabaseName,
    BrokerName
}

#[derive(Debug, Clone, Configure)]
pub enum OtherSampleConfig {
    OtherSettings
}


pub fn init(){
    SampleConfig::DatabaseName.set("test db name");
    SampleConfig::BrokerName.set("test broker name");

    OtherSampleConfig::OtherSettings.set("test other");
}