use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3529104613: FileType = FileType {
    file_format: &FileFormat {
        id: 3_529_104_613,
        source_type: SourceType::Iana,
        name: "vnd.ntt-local.ogw_remote-access",
        extensions: &[],
        media_types: &["application/vnd.ntt-local.ogw_remote-access"],
        signatures: &[],
        related_formats: &[],
    },
};
