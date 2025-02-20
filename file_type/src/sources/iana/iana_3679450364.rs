use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3679450364: FileType = FileType {
    file_format: &FileFormat {
        id: 3_679_450_364,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcvideo-regroup+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcvideo-regroup+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
