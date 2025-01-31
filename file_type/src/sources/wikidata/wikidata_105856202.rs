use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856202: FileFormat = FileFormat {
    id: 105_856_202,
    puid: "wikidata/105856202",
    name: "Advanced DB Master data (v3.0)",
    extensions: &["d01"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2C, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
