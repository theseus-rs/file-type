use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3083364676: FileFormat = FileFormat {
    id: 3_083_364_676,
    source_type: SourceType::Iana,
    name: "vnd.dtg.local",
    extensions: &[],
    media_types: &["application/vnd.dtg.local"],
    internal_signatures: &[],
    related_formats: &[],
};
