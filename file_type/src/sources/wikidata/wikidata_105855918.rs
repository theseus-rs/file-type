use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855918: FileFormat = FileFormat {
    id: 105_855_918,
    puid: "wikidata/105855918",
    name: "VariCAD Drawing (v9)",
    extensions: &["dwb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x87, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
