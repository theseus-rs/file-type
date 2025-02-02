use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854079: FileFormat = FileFormat {
    id: 105_854_079,
    source_type: SourceType::Wikidata,
    name: "Sonic Foundry Acid Project",
    extensions: &["acd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x69, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
