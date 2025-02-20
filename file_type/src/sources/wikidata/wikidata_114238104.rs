use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114238104: FileType = FileType {
    file_format: &FileFormat {
        id: 114_238_104,
        source_type: SourceType::Wikidata,
        name: "Netscape packetized audio",
        extensions: &["la", "lma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
