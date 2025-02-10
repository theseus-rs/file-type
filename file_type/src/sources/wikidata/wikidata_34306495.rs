use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34306495: FileType = FileType {
    file_format: &FileFormat {
        id: 34_306_495,
        source_type: SourceType::Wikidata,
        name: "Scifer archive",
        extensions: &["sen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
