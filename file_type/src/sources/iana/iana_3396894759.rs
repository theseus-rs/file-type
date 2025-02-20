use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3396894759: FileType = FileType {
    file_format: &FileFormat {
        id: 3_396_894_759,
        source_type: SourceType::Iana,
        name: "rpki-signed-tal",
        extensions: &[],
        media_types: &["application/rpki-signed-tal"],
        signatures: &[],
        related_formats: &[],
    },
};
