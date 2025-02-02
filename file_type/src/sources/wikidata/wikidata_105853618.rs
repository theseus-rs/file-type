use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853618: FileFormat = FileFormat {
    id: 105_853_618,
    source_type: SourceType::Wikidata,
    name: "Z80 music code with AY music",
    extensions: &["aym"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x59, 0x4D, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
