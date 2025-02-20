use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119443806: FileType = FileType {
    file_format: &FileFormat {
        id: 119_443_806,
        source_type: SourceType::Wikidata,
        name: "Map Template File",
        extensions: &["axt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
