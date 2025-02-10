use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66439263: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_263,
        source_type: SourceType::Wikidata,
        name: "Word Perfect Templates file format",
        extensions: &["wpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
