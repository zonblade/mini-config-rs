# mini-config-rs
Minimalistic Dynamic Config for Rust

### what's new on 0.1.2?
fixed memory leak. now it's safe for 2k RPS.\
[memory leak issue story [FIXED]](https://github.com/zonblade/mini-config-rs/issues/1)

### why this exist?
i've tired to try rust config out there. but it's too big and heavy for my small project.\
and for some reason, there is much feature which i did not use at all.\
so i decided to create a minimalistic config which uses enum to store data into memory.\
and retrive it using enum directly.

### how to use?
please add derive feature into Cargo.toml\
to enable `Configure` derive.
```toml
[dependencies]
mini-config = { version="*", features=["derive"]}
```

it can be used directly on main.
```rs
use mini_config::Configure;

// Debug and Clone "MUST" present.
#[derive(Debug,Clone,Configure)]
pub enum SampleConfig {
    VariableOne
}

fn main(){
    // set value (as &str)
    SampleConfig::VariableOne.set("string"); 
    
    // get value (as String)
    let retrived_value = SampleConfig::VariableOne.val();
}
```
if you want to call it in other function? just import the enum.
```rs
use crate::SampleConfig;
```
and you good to go using `.set("new val")` if you want to re-initialize\
or `.val()` if you want to get the data in that particular function.



best practice? please refer to `example` folder at github repository.