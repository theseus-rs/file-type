use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864740: FileFormat = FileFormat {
    id: 105_864_740,
    puid: "wikidata/105864740",
    name: "Expressware Printer Definition File",
    extensions: &["pdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x77, 0x61, 0x72, 0x65, 0x20, 0x50,
                    0x44, 0x46, 0x2D, 0x30, 0x31, 0x0D, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
