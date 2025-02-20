use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_30102323: FileType = FileType {
    file_format: &FileFormat {
        id: 30_102_323,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, ADI variant, version 3.0.5",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
