use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4105604214: FileType = FileType {
    file_format: &FileFormat {
        id: 4_105_604_214,
        source_type: SourceType::Iana,
        name: "vnd.veraison.tsm-report+cbor",
        extensions: &[],
        media_types: &["application/vnd.veraison.tsm-report+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
