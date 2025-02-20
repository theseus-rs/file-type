use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979224: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_224,
        source_type: SourceType::Wikidata,
        name: "Advanced Video Attribute Terminal Assembler and Recreator",
        extensions: &["avt", "bbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
