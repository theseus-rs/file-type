use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112661259: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_259,
        source_type: SourceType::Wikidata,
        name: "LightWave LScript File",
        extensions: &["ls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
