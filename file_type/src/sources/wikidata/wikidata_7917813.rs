use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7917813: FileType = FileType {
    file_format: &FileFormat {
        id: 7_917_813,
        source_type: SourceType::Wikidata,
        name: "Vector Product Format",
        extensions: &["vpf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
