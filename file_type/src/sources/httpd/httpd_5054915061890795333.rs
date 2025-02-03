use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5054915061890795333: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "relax ng compact syntax",
    extensions: &["rnc"],
    media_types: &["application/relax-ng-compact-syntax"],
    internal_signatures: &[],
    related_formats: &[],
};
