use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857222: FileFormat = FileFormat {
    id: 105_857_222,
    puid: "wikidata/105857222",
    name: "Hidden Agenda save game",
    extensions: &["ha"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60, 0xD7, 0x25, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
