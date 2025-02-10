use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112685424: FileType = FileType {
    file_format: &FileFormat {
        id: 112_685_424,
        source_type: SourceType::Wikidata,
        name: "3D Studio (DOS) project-file format",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
