use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855397: FileFormat = FileFormat {
    id: 105_855_397,
    source_type: SourceType::Wikidata,
    name: "Flowgorithm Color scheme",
    extensions: &["fclr"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4D, 0x61, 0x69, 0x6E, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
