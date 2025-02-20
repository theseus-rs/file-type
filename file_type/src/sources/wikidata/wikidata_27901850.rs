use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27901850: FileType = FileType {
    file_format: &FileFormat {
        id: 27_901_850,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, version 2.1.5",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
