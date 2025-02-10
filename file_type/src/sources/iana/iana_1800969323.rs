use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1800969323: FileType = FileType {
    file_format: &FileFormat {
        id: 1_800_969_323,
        source_type: SourceType::Iana,
        name: "vnd.ecowin.seriesrequest",
        extensions: &[],
        media_types: &["application/vnd.ecowin.seriesrequest"],
        signatures: &[],
        related_formats: &[],
    },
};
