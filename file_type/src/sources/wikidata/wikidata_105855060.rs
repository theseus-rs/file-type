use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855060: FileFormat = FileFormat {
    id: 105_855_060,
    source_type: SourceType::Wikidata,
    name: "Abyss Engine Image format",
    extensions: &["aei"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x45, 0x69, 0x6D, 0x61, 0x67, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
