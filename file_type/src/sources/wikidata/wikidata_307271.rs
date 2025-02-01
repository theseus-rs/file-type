use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_307271: FileFormat = FileFormat {
    id: 307_271,
    puid: "wikidata/307271",
    name: ".DS_Store",
    extensions: &["DS_Store"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x01, 0x42, 0x75, 0x64, 0x31, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
