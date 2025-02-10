use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3093602065: FileType = FileType {
    file_format: &FileFormat {
        id: 3_093_602_065,
        source_type: SourceType::Iana,
        name: "vnd.cncf.helm.chart.provenance.v1.prov",
        extensions: &[],
        media_types: &["application/vnd.cncf.helm.chart.provenance.v1.prov"],
        signatures: &[],
        related_formats: &[],
    },
};
