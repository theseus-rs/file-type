use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205987: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_987,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Monochrome",
        extensions: &["imm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
