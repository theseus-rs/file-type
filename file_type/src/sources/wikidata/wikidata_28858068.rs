use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28858068: FileType = FileType {
    file_format: &FileFormat {
        id: 28_858_068,
        source_type: SourceType::Wikidata,
        name: "Binary Interchange File Format, version 8",
        extensions: &["xls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
