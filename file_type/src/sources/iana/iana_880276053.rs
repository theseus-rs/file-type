use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_880276053: FileFormat = FileFormat {
    id: 880_276_053,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.tunnel-udpencap",
    extensions: &[],
    media_types: &["application/vnd.yamaha.tunnel-udpencap"],
    internal_signatures: &[],
    related_formats: &[],
};
