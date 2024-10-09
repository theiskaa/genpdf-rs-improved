#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ResourceId {
Enusstandardbincode,
}
impl ResourceId {
    #[inline]
    pub fn load(&self) -> &'static [u8] {
        match self {

                &ResourceId::Enusstandardbincode => &include_bytes!(r#"/Users/i.shahverdiev/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hyphenation-0.8.4/dictionaries/en-us.standard.bincode"#)[..],
            
        }
    }
    #[inline]
    pub fn name_ext(&self) -> &str {
        match *self {

                ResourceId::Enusstandardbincode => r#"en-us.standard.bincode"#,
            
        }
    }
    #[inline]
    pub fn from_name(name: &str) -> Option<ResourceId> {

                if name == r#"en-us.standard.bincode"# { return Some(ResourceId::Enusstandardbincode); }
            

                    if name == r#"en-us.standard"# { return Some(ResourceId::Enusstandardbincode); }
                
        None
    }
}
#[allow(missing_docs)] pub const ENUS_STANDARD_BINCODE: ResourceId = ResourceId::Enusstandardbincode;
#[allow(missing_docs)] pub const ENUS_STANDARD: ResourceId = ResourceId::Enusstandardbincode;
