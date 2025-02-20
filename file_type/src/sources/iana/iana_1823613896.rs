use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1823613896: FileType = FileType {
    file_format: &FileFormat {
        id: 1_823_613_896,
        source_type: SourceType::Iana,
        name: "vnd.oipf.spdiscovery+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.spdiscovery+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
