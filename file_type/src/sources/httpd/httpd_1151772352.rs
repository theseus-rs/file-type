use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1151772352: FileFormat = FileFormat {
    id: 1_151_772_352,
    source_type: SourceType::Httpd,
    name: "install instructions",
    extensions: &["install"],
    media_types: &["application/x-install-instructions"],
    signatures: &[],
    related_formats: &[],
};
