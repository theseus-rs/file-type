use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111417314: FileType = FileType {
    file_format: &FileFormat {
        id: 111_417_314,
        source_type: SourceType::Wikidata,
        name: "Borland Turbo C++ Project File",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
