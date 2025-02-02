use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853257: FileFormat = FileFormat {
    id: 105_853_257,
    source_type: SourceType::Wikidata,
    name: "Hitec Aurora 9 saved model",
    extensions: &["set"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x56,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x30, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
