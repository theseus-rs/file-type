use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_334794876: FileType = FileType {
    file_format: &FileFormat {
        id: 334_794_876,
        source_type: SourceType::Iana,
        name: "vnd.cncf.helm.chart.content.v1.tar+gzip",
        extensions: &[],
        media_types: &["application/vnd.cncf.helm.chart.content.v1.tar+gzip"],
        signatures: &[],
        related_formats: &[],
    },
};
