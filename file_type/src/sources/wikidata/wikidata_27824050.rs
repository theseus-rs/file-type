use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27824050: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_050,
        source_type: SourceType::Wikidata,
        name: "ar, AIX big archive format variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
