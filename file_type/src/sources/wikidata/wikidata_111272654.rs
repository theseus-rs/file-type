use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272654: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_654,
        source_type: SourceType::Wikidata,
        name: "ESPS audio file",
        extensions: &["esps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
