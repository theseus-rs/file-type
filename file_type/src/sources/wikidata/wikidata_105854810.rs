use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854810: FileFormat = FileFormat {
    id: 105_854_810,
    source_type: SourceType::Wikidata,
    name: "Windows Policy Administrative Template (with rem)",
    extensions: &["adm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
