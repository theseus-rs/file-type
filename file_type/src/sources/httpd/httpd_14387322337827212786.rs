use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14387322337827212786: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fujitsu oasysgp",
    extensions: &["fg5"],
    media_types: &["application/vnd.fujitsu.oasysgp"],
    internal_signatures: &[],
    related_formats: &[],
};
