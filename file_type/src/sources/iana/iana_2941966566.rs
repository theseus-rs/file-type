use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2941966566: FileType = FileType {
    file_format: &FileFormat {
        id: 2_941_966_566,
        source_type: SourceType::Iana,
        name: "prs.texi",
        extensions: &[],
        media_types: &["text/prs.texi"],
        signatures: &[],
        related_formats: &[],
    },
};
