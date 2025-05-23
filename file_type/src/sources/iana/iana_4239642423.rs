use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4239642423: FileType = FileType {
    file_format: &FileFormat {
        id: 4_239_642_423,
        source_type: SourceType::Iana,
        name: "vnd.etsi.mcid+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.mcid+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
