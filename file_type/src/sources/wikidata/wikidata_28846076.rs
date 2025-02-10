use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28846076: FileType = FileType {
    file_format: &FileFormat {
        id: 28_846_076,
        source_type: SourceType::Wikidata,
        name: "Classification Results File Format",
        extensions: &["clr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
