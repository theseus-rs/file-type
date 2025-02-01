use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853791: FileFormat = FileFormat {
    id: 105_853_791,
    puid: "wikidata/105853791",
    name: "Jupiter Ace snapshot",
    extensions: &["ace"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
