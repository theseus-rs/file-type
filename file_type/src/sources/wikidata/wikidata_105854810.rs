use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854810: FileFormat = FileFormat {
    id: 105_854_810,
    source_type: SourceType::Wikidata,
    name: "Windows Policy Administrative Template (with rem)",
    extensions: &["adm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
