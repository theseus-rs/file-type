use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1349874601: FileType = FileType {
    file_format: &FileFormat {
        id: 1_349_874_601,
        source_type: SourceType::Iana,
        name: "vnd.ecowin.chart",
        extensions: &[],
        media_types: &["application/vnd.ecowin.chart"],
        signatures: &[],
        related_formats: &[],
    },
};
