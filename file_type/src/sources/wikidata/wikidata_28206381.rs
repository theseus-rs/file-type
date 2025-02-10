use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206381: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_381,
        source_type: SourceType::Wikidata,
        name: "VisualBasic form",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
