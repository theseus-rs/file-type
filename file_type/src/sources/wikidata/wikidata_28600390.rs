use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600390: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_390,
        source_type: SourceType::Wikidata,
        name: "Apple framework",
        extensions: &["framework"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
