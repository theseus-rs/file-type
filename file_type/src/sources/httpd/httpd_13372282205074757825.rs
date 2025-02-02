use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13372282205074757825: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "blorb",
    extensions: &["blb", "blorb"],
    media_types: &["application/x-blorb"],
    internal_signatures: &[],
    related_formats: &[],
};
