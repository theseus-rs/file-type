use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29904535: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_535,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System program",
        extensions: &["sas"],
        media_types: &[
            "application/octet-stream",
            "application/x-sas",
            "text/sas",
            "text/x-sas",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
