use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_334794876: FileFormat = FileFormat {
    id: 334_794_876,
    source_type: SourceType::Iana,
    name: "vnd.cncf.helm.chart.content.v1.tar+gzip",
    extensions: &[],
    media_types: &["application/vnd.cncf.helm.chart.content.v1.tar+gzip"],
    internal_signatures: &[],
    related_formats: &[],
};
