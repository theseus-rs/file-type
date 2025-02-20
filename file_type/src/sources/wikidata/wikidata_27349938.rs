use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27349938: FileType = FileType {
    file_format: &FileFormat {
        id: 27_349_938,
        source_type: SourceType::Wikidata,
        name: "TOPSAR Digital Elevation Model",
        extensions: &["demi2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
