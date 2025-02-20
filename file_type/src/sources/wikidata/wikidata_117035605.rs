use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117035605: FileType = FileType {
    file_format: &FileFormat {
        id: 117_035_605,
        source_type: SourceType::Wikidata,
        name: "VRML geography data",
        extensions: &["geo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
