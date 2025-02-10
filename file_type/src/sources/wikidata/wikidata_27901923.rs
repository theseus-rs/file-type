use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27901923: FileType = FileType {
    file_format: &FileFormat {
        id: 27_901_923,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, version 2.2.5",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
