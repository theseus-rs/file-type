use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125049964: FileType = FileType {
    file_format: &FileFormat {
        id: 125_049_964,
        source_type: SourceType::Wikidata,
        name: "Yoshimi Vector settings file",
        extensions: &["xvy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
