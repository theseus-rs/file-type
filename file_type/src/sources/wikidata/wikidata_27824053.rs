use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27824053: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_053,
        source_type: SourceType::Wikidata,
        name: "ar, AIX small archive format variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
