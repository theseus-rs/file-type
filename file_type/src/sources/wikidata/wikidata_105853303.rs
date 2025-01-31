use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853303: FileFormat = FileFormat {
    id: 105_853_303,
    puid: "wikidata/105853303",
    name: "SuperTux Level",
    extensions: &["stl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x73, 0x75, 0x70, 0x65, 0x72, 0x74, 0x75, 0x78, 0x2D, 0x6C, 0x65, 0x76,
                    0x65, 0x6C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
