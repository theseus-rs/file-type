use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111330884: FileType = FileType {
    file_format: &FileFormat {
        id: 111_330_884,
        source_type: SourceType::Wikidata,
        name: "Musifile MPEG Layer II audio stream",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
