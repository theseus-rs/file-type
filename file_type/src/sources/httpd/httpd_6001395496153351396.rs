use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6001395496153351396: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "iges",
    extensions: &["igs", "iges"],
    media_types: &["model/iges"],
    internal_signatures: &[],
    related_formats: &[],
};
