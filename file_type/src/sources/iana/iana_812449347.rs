use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_812449347: FileFormat = FileFormat {
    id: 812_449_347,
    source_type: SourceType::Iana,
    name: "vnd.yamaha.openscoreformat.osfpvg+xml",
    extensions: &[],
    media_types: &["application/vnd.yamaha.openscoreformat.osfpvg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
