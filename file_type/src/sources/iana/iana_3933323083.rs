use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3933323083: FileType = FileType {
    file_format: &FileFormat {
        id: 3_933_323_083,
        source_type: SourceType::Iana,
        name: "1d-interleaved-parityfec",
        extensions: &[],
        media_types: &["audio/1d-interleaved-parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
