use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853684: FileFormat = FileFormat {
    id: 105_853_684,
    puid: "wikidata/105853684",
    name: "AUTOMGEN project",
    extensions: &["agn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x47, 0x4E, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x41, 0x55, 0x54, 0x4F, 0x4D,
                    0x47, 0x45, 0x4E, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
