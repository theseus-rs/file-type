use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3678724516: FileFormat = FileFormat {
    id: 3_678_724_516,
    source_type: SourceType::Httpd,
    name: "dvb file",
    extensions: &["dvb"],
    media_types: &["video/vnd.dvb.file"],
    signatures: &[],
    related_formats: &[],
};
