use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110098601: FileType = FileType {
    file_format: &FileFormat {
        id: 110_098_601,
        source_type: SourceType::Wikidata,
        name: "EinScan RGE 3D Range File",
        extensions: &["rge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
