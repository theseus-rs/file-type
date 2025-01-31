use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859067: FileFormat = FileFormat {
    id: 105_859_067,
    puid: "wikidata/105859067",
    name: "ImageLab bitmap",
    extensions: &["b_w"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x26, 0x57, 0x32, 0x35, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
