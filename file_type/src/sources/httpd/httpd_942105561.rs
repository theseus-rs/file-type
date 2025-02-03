use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_942105561: FileFormat = FileFormat {
    id: 942_105_561,
    source_type: SourceType::Httpd,
    name: "rpki manifest",
    extensions: &["mft"],
    media_types: &["application/rpki-manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
