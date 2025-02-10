use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_379967318: FileType = FileType {
    file_format: &FileFormat {
        id: 379_967_318,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcvideo-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcvideo-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
