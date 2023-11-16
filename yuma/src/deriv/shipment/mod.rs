use crate::{deriv::packager::GenericName, prelude::*};

use std::{cell::LazyCell, path::Path};

use super::packager::{PackagerType, SpecficName};

thread_local! {
pub static SHIPYARD: LazyCell<Shipyard> = LazyCell::new(|| Shipyard::new(".").unwrap());
}

#[derive(Debug)]
struct Shipment {
    name: super::packager::GenericName,
    packagers: Vec<(PackagerType, ShipmentData)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShipmentData {
    name: SpecficName,
}

#[derive(Debug)]
pub struct Shipyard {
    shipments: Vec<Shipment>,
}

impl Shipyard {
    pub const fn empty() -> Self {
        Self {
            shipments: Vec::new(),
        }
    }

    /// Creates a new [`Shipyard`] and adds a base registry to it.
    pub fn new<P>(registry: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut ret = Self::empty();

        ret.add_registry(registry)?;

        Ok(ret)
    }

    /// This serializes data across the file system with the format of
    ///
    /// root_dir
    /// L__. packager1
    /// |  L__ pkg1
    /// |  L__ pkg2
    /// |  L__ pkg4
    /// L__. packager2
    /// |  L__ pkg2
    /// |  L__ pkg3
    /// L__. packager3
    ///    L__ pkg3
    ///    L__ pkg4
    pub fn add_registry<P: AsRef<Path>>(&mut self, root: P) -> Result<()> {
        for dir in fs::read_dir(root)? {
            let dir = dir?;
            if dir.file_type()?.is_dir() {
                let path = dir.path();
                let ptype: PackagerType = dir.file_name().to_str().unwrap().try_into()?;
                info!("Registry for packager found at {}", path.display());
                for file in fs::read_dir(path)? {
                    let file = file?;
                    if file.file_type()?.is_file() {
                        let name = GenericName::new(file.file_name().to_str().unwrap().to_owned());
                        let f = fs::File::open(file.path())?;
                        let data: ShipmentData = json::from_reader(f)?;
                        if let Some(sp) = self.shipments.iter_mut().find(|sp| sp.name == name) {
                            sp.packagers.push((ptype, data))
                        } else {
                            self.shipments.push(Shipment {
                                name,
                                packagers: vec![(ptype, data)],
                            })
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
