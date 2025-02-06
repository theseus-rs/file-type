use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2028473068: FileFormat = FileFormat {
    id: 2_028_473_068,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-dialog-base+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-dialog-base+xml"],
    signatures: &[],
    related_formats: &[],
};
