use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128780753: FileFormat = FileFormat {
    id: 128_780_753,
    source_type: SourceType::Wikidata,
    name: "crmsh configuration file",
    extensions: &["crmsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
