use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1399920820: FileType = FileType {
    file_format: &FileFormat {
        id: 1_399_920_820,
        source_type: SourceType::Iana,
        name: "prs.vcfbzip2",
        extensions: &[],
        media_types: &["application/prs.vcfbzip2"],
        signatures: &[],
        related_formats: &[],
    },
};
