use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2777192593731638264: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "visio",
    extensions: &["vsd", "vst", "vss", "vsw"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
