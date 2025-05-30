use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67206685: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_685,
        source_type: SourceType::Wikidata,
        name: "FastCAD DOS file",
        extensions: &["fcd"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/ffd72a57f5f0205d2a8bb8294151d36b",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
