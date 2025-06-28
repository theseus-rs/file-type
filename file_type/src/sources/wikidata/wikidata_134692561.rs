use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134692561: FileType = FileType {
    file_format: &FileFormat {
        id: 134_692_561,
        source_type: SourceType::Wikidata,
        name: "NooJ project file",
        extensions: &["nop"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
