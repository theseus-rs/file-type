use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856277: FileFormat = FileFormat {
    id: 105_856_277,
    puid: "wikidata/105856277",
    name: "VariCAD Drawing (v7)",
    extensions: &["dwb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x87, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
