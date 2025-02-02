use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14460022341763891094: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "s3m",
    extensions: &["s3m"],
    media_types: &["audio/s3m"],
    internal_signatures: &[],
    related_formats: &[],
};
