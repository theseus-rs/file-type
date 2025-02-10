use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3062216814: FileType = FileType {
    file_format: &FileFormat {
        id: 3_062_216_814,
        source_type: SourceType::Iana,
        name: "prs.mayfile",
        extensions: &[],
        media_types: &["application/prs.mayfile"],
        signatures: &[],
        related_formats: &[],
    },
};
