use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100344893: FileType = FileType {
    file_format: &FileFormat {
        id: 100_344_893,
        source_type: SourceType::Wikidata,
        name: "Corel Photo House Image",
        extensions: &["cps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
