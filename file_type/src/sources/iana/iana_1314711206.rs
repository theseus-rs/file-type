use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1314711206: FileType = FileType {
    file_format: &FileFormat {
        id: 1_314_711_206,
        source_type: SourceType::Iana,
        name: "vnd.pvi.ptid1",
        extensions: &[],
        media_types: &["application/vnd.pvi.ptid1"],
        signatures: &[],
        related_formats: &[],
    },
};
