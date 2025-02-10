use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_919687982: FileType = FileType {
    file_format: &FileFormat {
        id: 919_687_982,
        source_type: SourceType::Iana,
        name: "alto-endpointcost+json",
        extensions: &[],
        media_types: &["application/alto-endpointcost+json"],
        signatures: &[],
        related_formats: &[],
    },
};
