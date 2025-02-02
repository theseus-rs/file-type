use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860755: FileFormat = FileFormat {
    id: 105_860_755,
    source_type: SourceType::Wikidata,
    name: "Pokemon Randomization Quick Settings",
    extensions: &["rnqs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
