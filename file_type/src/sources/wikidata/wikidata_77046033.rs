use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_77046033: FileType = FileType {
    file_format: &FileFormat {
        id: 77_046_033,
        source_type: SourceType::Wikidata,
        name: "Extensible 3D vector graphics (VRML)",
        extensions: &["x3dv"],
        media_types: &["model/x3d-vrml"],
        signatures: &[],
        related_formats: &[],
    },
};
