use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47012501: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_501,
        source_type: SourceType::Wikidata,
        name: "OmniPage Pro Document file format",
        extensions: &["met", "opd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
