use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113674382: FileType = FileType {
    file_format: &FileFormat {
        id: 113_674_382,
        source_type: SourceType::Wikidata,
        name: "File Converter Document",
        extensions: &["fcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
