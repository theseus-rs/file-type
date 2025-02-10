use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131543477: FileType = FileType {
    file_format: &FileFormat {
        id: 131_543_477,
        source_type: SourceType::Wikidata,
        name: "Varian FDF file format",
        extensions: &["fdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
