use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27901919: FileType = FileType {
    file_format: &FileFormat {
        id: 27_901_919,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, version 2.2.1",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
