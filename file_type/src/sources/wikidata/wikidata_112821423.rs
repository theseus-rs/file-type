use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112821423: FileType = FileType {
    file_format: &FileFormat {
        id: 112_821_423,
        source_type: SourceType::Wikidata,
        name: "Minolta 3D Scanner Element File",
        extensions: &["vvd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
