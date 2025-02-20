use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2986419294: FileType = FileType {
    file_format: &FileFormat {
        id: 2_986_419_294,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca"],
        signatures: &[],
        related_formats: &[],
    },
};
