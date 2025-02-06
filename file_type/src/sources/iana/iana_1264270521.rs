use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1264270521: FileFormat = FileFormat {
    id: 1_264_270_521,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.packageitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.packageitem+xml"],
    signatures: &[],
    related_formats: &[],
};
