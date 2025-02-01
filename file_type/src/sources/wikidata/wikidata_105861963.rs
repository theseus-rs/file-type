use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861963: FileFormat = FileFormat {
    id: 105_861_963,
    puid: "wikidata/105861963",
    name: "My CEWE Photobook project",
    extensions: &["mcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x66, 0x6F, 0x74, 0x6F, 0x62, 0x6F, 0x6F, 0x6B, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
