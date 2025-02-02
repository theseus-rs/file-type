use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16270552558721564289: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rpki manifest",
    extensions: &["mft"],
    media_types: &["application/rpki-manifest"],
    internal_signatures: &[],
    related_formats: &[],
};
