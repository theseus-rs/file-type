use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3800263245: FileFormat = FileFormat {
    id: 3_800_263_245,
    source_type: SourceType::Httpd,
    name: "msmetafile",
    extensions: &["wmf", "wmz", "emf", "emz"],
    media_types: &["application/x-msmetafile"],
    internal_signatures: &[],
    related_formats: &[],
};
