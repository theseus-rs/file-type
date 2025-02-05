use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116785245: FileFormat = FileFormat {
    id: 116_785_245,
    source_type: SourceType::Wikidata,
    name: "602Pro PC Suite macro",
    extensions: &["cnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
