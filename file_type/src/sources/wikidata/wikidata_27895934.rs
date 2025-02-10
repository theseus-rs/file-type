use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27895934: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_934,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, version 1",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
