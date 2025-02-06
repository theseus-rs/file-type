use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67383807: FileFormat = FileFormat {
    id: 67_383_807,
    source_type: SourceType::Wikidata,
    name: "ASCII Header Format bitmap",
    extensions: &["ahf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x48, 0x46, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
