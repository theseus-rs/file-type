use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59390863: FileType = FileType {
    file_format: &FileFormat {
        id: 59_390_863,
        source_type: SourceType::Wikidata,
        name: "Domino XML Database Export",
        extensions: &["dxl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
