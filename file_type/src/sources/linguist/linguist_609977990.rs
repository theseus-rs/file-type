use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_609977990: FileFormat = FileFormat {
    id: 609_977_990,
    source_type: SourceType::Linguist,
    name: "RPGLE",
    extensions: &["rpgle", "sqlrpgle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
