use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3022955634: FileFormat = FileFormat {
    id: 3_022_955_634,
    source_type: SourceType::Httpd,
    name: "novadigm edm",
    extensions: &["edm"],
    media_types: &["application/vnd.novadigm.edm"],
    internal_signatures: &[],
    related_formats: &[],
};
