use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133258216: FileType = FileType {
    file_format: &FileFormat {
        id: 133_258_216,
        source_type: SourceType::Wikidata,
        name: "Daisy Talking Book Navigation Control File 3",
        extensions: &["ncx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
