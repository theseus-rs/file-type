use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_75717634: FileType = FileType {
    file_format: &FileFormat {
        id: 75_717_634,
        source_type: SourceType::Wikidata,
        name: "Corel Photo Paint User Defined Filter",
        extensions: &["usr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
