use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2269607461: FileFormat = FileFormat {
    id: 2_269_607_461,
    source_type: SourceType::Httpd,
    name: "fujixerox ddd",
    extensions: &["ddd"],
    media_types: &["application/vnd.fujixerox.ddd"],
    internal_signatures: &[],
    related_formats: &[],
};
