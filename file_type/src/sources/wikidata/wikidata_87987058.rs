use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87987058: FileType = FileType {
    file_format: &FileFormat {
        id: 87_987_058,
        source_type: SourceType::Wikidata,
        name: "CorelCHART Document 5",
        extensions: &["cch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
