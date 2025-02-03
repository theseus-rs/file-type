use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_379967318: FileFormat = FileFormat {
    id: 379_967_318,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
