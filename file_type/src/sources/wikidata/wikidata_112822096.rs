use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112822096: FileType = FileType {
    file_format: &FileFormat {
        id: 112_822_096,
        source_type: SourceType::Wikidata,
        name: "Strata StudioPro 3D File, version 1.75",
        extensions: &["vis"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
