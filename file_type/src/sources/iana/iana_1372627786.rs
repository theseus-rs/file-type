use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1372627786: FileType = FileType {
    file_format: &FileFormat {
        id: 1_372_627_786,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcvideo-affiliation-command+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcvideo-affiliation-command+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
