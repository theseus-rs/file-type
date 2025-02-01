use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856814: FileFormat = FileFormat {
    id: 105_856_814,
    puid: "wikidata/105856814",
    name: "Arts and Letters Graphics file",
    extensions: &["ged"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x26, 0x4C, 0x2D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
