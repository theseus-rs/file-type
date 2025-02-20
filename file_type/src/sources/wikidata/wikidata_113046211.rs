use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113046211: FileType = FileType {
    file_format: &FileFormat {
        id: 113_046_211,
        source_type: SourceType::Wikidata,
        name: "Live Code File Format",
        extensions: &["mlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
