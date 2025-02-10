use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979322: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_322,
        source_type: SourceType::Wikidata,
        name: "Photoshop Curve",
        extensions: &["crv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
