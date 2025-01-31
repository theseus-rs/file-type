use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856738: FileFormat = FileFormat {
    id: 105_856_738,
    puid: "wikidata/105856738",
    name: "Universal Hint System (old 88a format)",
    extensions: &["uhs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x48, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
