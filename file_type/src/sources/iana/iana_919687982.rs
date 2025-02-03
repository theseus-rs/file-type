use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_919687982: FileFormat = FileFormat {
    id: 919_687_982,
    source_type: SourceType::Iana,
    name: "alto-endpointcost+json",
    extensions: &[],
    media_types: &["application/alto-endpointcost+json"],
    internal_signatures: &[],
    related_formats: &[],
};
