use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109944655: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_655,
        source_type: SourceType::Wikidata,
        name: "Image Systems file format",
        extensions: &["igs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
