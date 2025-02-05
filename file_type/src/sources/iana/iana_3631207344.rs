use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3631207344: FileFormat = FileFormat {
    id: 3_631_207_344,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-audit-dialog+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-audit-dialog+xml"],
    signatures: &[],
    related_formats: &[],
};
