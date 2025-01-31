use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855084: FileFormat = FileFormat {
    id: 105_855_084,
    puid: "wikidata/105855084",
    name: "AMGC compressed archive",
    extensions: &["amg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAD, 0x36, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94,
                    0x1C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
