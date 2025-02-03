use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1215761775: FileFormat = FileFormat {
    id: 1_215_761_775,
    source_type: SourceType::Httpd,
    name: "3gpp pic bw large",
    extensions: &["plb"],
    media_types: &["application/vnd.3gpp.pic-bw-large"],
    internal_signatures: &[],
    related_formats: &[],
};
