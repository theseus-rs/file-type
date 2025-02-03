use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9209038509344480468: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "airzip filesecure azs",
    extensions: &["azs"],
    media_types: &["application/vnd.airzip.filesecure.azs"],
    internal_signatures: &[],
    related_formats: &[],
};
