use std::{fs::File, io::{BufReader, Read, Seek}, path::Path};

use xml::{EventReader, reader::XmlEvent};

use crate::glue::patch::SurgePatch;

const XML_OFFSET: u64 = 0x5c;

pub struct EasyPatch {
    patch: SurgePatch,
}

impl EasyPatch {
    pub fn new() -> Self {
        Self { patch: SurgePatch::new() }
    }

    pub fn from_fpx(path: &Path) -> Self {  // helper.
        let mut me = EasyPatch::new();
        me.load_fpx(path);
        me
    }

    pub fn load_fpx(&mut self, path: &Path) {
        let mut fpx = File::open(path).expect("could not open file.");  // add more info?
        fpx.seek(std::io::SeekFrom::Start(XML_OFFSET)).expect("seek failed.");

        let mut buffer = Vec::new();
        fpx.read_to_end(&mut buffer).expect("read failed.");

        self.patch.load_xml(&buffer, false);
    }
}
