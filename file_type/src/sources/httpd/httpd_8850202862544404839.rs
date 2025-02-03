use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8850202862544404839: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms ims",
    extensions: &["ims"],
    media_types: &["application/vnd.ms-ims"],
    internal_signatures: &[],
    related_formats: &[],
};
