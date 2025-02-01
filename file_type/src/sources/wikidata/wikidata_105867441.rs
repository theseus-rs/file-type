use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867441: FileFormat = FileFormat {
    id: 105_867_441,
    puid: "wikidata/105867441",
    name: "Nokia PC Suite Content Copier file",
    extensions: &["ncc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x32, 0x00, 0x30, 0x00, 0x30, 0x00, 0x09, 0x00, 0x50, 0x00, 0x49,
                    0x00, 0x54, 0x00, 0x5F, 0x00, 0x43, 0x00, 0x4F, 0x00, 0x4E, 0x00, 0x54, 0x00,
                    0x41, 0x00, 0x43, 0x00, 0x54, 0x00, 0x09, 0x00, 0x32, 0x00, 0x30, 0x00, 0x32,
                    0x00, 0x09, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
