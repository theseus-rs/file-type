use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851821: FileFormat = FileFormat {
    id: 105_851_821,
    puid: "wikidata/105851821",
    name: "Artline Symbol File",
    extensions: &["syf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x43, 0x50, 0x41, 0x30, 0x30, 0x32, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
