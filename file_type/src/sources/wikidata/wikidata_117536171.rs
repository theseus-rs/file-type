use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117536171: FileType = FileType {
    file_format: &FileFormat {
        id: 117_536_171,
        source_type: SourceType::Wikidata,
        name: "3D Studio (DOS) 2D/3D Loft Object File",
        extensions: &["lft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
