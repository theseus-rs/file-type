use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3477347748: FileFormat = FileFormat {
    id: 3_477_347_748,
    source_type: SourceType::Httpd,
    name: "java source",
    extensions: &["java"],
    media_types: &["text/x-java-source"],
    internal_signatures: &[],
    related_formats: &[],
};
