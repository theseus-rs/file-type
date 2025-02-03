use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2610229657: FileFormat = FileFormat {
    id: 2_610_229_657,
    source_type: SourceType::Iana,
    name: "vnd.bbf.usp.msg",
    extensions: &[],
    media_types: &["application/vnd.bbf.usp.msg"],
    internal_signatures: &[],
    related_formats: &[],
};
