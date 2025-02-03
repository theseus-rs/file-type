use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9943181513677621699: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument spreadsheet",
    extensions: &["ods"],
    media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
    internal_signatures: &[],
    related_formats: &[],
};
