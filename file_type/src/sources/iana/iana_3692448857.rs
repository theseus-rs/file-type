use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3692448857: FileType = FileType {
    file_format: &FileFormat {
        id: 3_692_448_857,
        source_type: SourceType::Iana,
        name: "vnd.pg.osasli",
        extensions: &[],
        media_types: &["application/vnd.pg.osasli"],
        signatures: &[],
        related_formats: &[],
    },
};
