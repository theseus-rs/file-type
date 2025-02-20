use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205822: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_822,
        source_type: SourceType::Wikidata,
        name: "CD5",
        extensions: &["cd5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
