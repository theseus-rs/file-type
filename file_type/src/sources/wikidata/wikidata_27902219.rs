use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27902219: FileType = FileType {
    file_format: &FileFormat {
        id: 27_902_219,
        source_type: SourceType::Wikidata,
        name: "Amateur Data Interchange Format, ADI variant, version 3.0.3",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
