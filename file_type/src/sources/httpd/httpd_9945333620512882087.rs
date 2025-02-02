use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9945333620512882087: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "iso9660 image",
    extensions: &["iso"],
    media_types: &["application/x-iso9660-image"],
    internal_signatures: &[],
    related_formats: &[],
};
