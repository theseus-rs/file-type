use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854042: FileFormat = FileFormat {
    id: 105_854_042,
    puid: "wikidata/105854042",
    name: "AppleWin SaveState",
    extensions: &["aws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x57, 0x53, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
