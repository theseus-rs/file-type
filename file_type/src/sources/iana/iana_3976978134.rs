use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3976978134: FileType = FileType {
    file_format: &FileFormat {
        id: 3_976_978_134,
        source_type: SourceType::Iana,
        name: "vnd.oma.cab-pcc+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.cab-pcc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
