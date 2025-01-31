use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979411: FileFormat = FileFormat {
    id: 27_979_411,
    puid: "wikidata/27979411",
    name: "iCEDraw",
    extensions: &["idf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x04, 0x31, 0x2E, 0x34, 0x00, 0x00, 0x00, 0x00, 0x4F, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
