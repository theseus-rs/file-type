use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855855: FileFormat = FileFormat {
    id: 105_855_855,
    puid: "wikidata/105855855",
    name: "ideaMaker print preview data",
    extensions: &["data"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x44, 0x45, 0x41, 0x20, 0x2D, 0x20, 0x50, 0x52, 0x49, 0x4E, 0x54, 0x44,
                    0x41, 0x54, 0x41, 0x49, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
