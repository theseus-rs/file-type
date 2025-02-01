use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859238: FileFormat = FileFormat {
    id: 105_859_238,
    puid: "wikidata/105859238",
    name: "AmigaBASIC source (protected)",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF4, 0xC2])],
            },
        }],
    }],
    related_formats: &[],
};
