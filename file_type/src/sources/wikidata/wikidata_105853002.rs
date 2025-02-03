use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853002: FileFormat = FileFormat {
    id: 105_853_002,
    source_type: SourceType::Wikidata,
    name: "Datalink SoundScape",
    extensions: &["spc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x04, 0x19, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
