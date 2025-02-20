use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2465153865: FileType = FileType {
    file_format: &FileFormat {
        id: 2_465_153_865,
        source_type: SourceType::Iana,
        name: "vnd.oipf.pae.gem",
        extensions: &[],
        media_types: &["application/vnd.oipf.pae.gem"],
        signatures: &[],
        related_formats: &[],
    },
};
