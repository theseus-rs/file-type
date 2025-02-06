use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_667863728: FileFormat = FileFormat {
    id: 667_863_728,
    source_type: SourceType::Iana,
    name: "vnd.etsi.overload-control-policy-dataset+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.overload-control-policy-dataset+xml"],
    signatures: &[],
    related_formats: &[],
};
