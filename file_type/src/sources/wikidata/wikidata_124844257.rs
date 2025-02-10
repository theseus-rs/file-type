use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124844257: FileType = FileType {
    file_format: &FileFormat {
        id: 124_844_257,
        source_type: SourceType::Wikidata,
        name: "MediaShow Slideshow Project File",
        extensions: &["mse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
