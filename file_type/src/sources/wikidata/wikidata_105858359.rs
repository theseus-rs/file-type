use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858359: FileFormat = FileFormat {
    id: 105_858_359,
    source_type: SourceType::Wikidata,
    name: "E-Studio 1.x experiment",
    extensions: &["es"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
