use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34306592: FileType = FileType {
    file_format: &FileFormat {
        id: 34_306_592,
        source_type: SourceType::Wikidata,
        name: "Scifer archive binary header",
        extensions: &["ba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
