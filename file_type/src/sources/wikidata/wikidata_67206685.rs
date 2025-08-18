use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67206685: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_685,
        source_type: SourceType::Wikidata,
        name: "FastCAD DOS file",
        extensions: &["fcd"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/650d6c7450b6fc3aae5acf0b90172091",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
