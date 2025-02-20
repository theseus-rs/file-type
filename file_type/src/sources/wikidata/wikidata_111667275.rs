use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111667275: FileType = FileType {
    file_format: &FileFormat {
        id: 111_667_275,
        source_type: SourceType::Wikidata,
        name: "OEW objectbase",
        extensions: &["oew"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
