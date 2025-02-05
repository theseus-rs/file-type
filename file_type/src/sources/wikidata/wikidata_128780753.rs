use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128780753: FileFormat = FileFormat {
    id: 128_780_753,
    source_type: SourceType::Wikidata,
    name: "crmsh configuration file",
    extensions: &["crmsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
