use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87896505: FileType = FileType {
    file_format: &FileFormat {
        id: 87_896_505,
        source_type: SourceType::Wikidata,
        name: "Avery DesignPro Document 4",
        extensions: &["zdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
