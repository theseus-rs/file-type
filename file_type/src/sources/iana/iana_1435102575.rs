use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1435102575: FileFormat = FileFormat {
    id: 1_435_102_575,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.hv-dic",
    extensions: &[],
    media_types: &["application/vnd.yamaha.hv-dic"],
    internal_signatures: &[],
    related_formats: &[],
};
