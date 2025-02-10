use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119978112: FileType = FileType {
    file_format: &FileFormat {
        id: 119_978_112,
        source_type: SourceType::Wikidata,
        name: "Clip File",
        extensions: &["fmc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
