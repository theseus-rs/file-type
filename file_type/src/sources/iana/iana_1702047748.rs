use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1702047748: FileFormat = FileFormat {
    id: 1_702_047_748,
    source_type: SourceType::Iana,
    name: "vnd.las.las+xml",
    extensions: &[],
    media_types: &["application/vnd.las.las+xml"],
    signatures: &[],
    related_formats: &[],
};
