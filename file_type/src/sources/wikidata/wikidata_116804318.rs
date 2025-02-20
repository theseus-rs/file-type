use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116804318: FileType = FileType {
    file_format: &FileFormat {
        id: 116_804_318,
        source_type: SourceType::Wikidata,
        name: "TimeWiz Catalog File",
        extensions: &["twc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
