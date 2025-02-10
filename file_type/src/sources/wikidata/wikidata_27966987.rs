use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966987: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_987,
        source_type: SourceType::Wikidata,
        name: "AMOS Music Bank",
        extensions: &["abk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
