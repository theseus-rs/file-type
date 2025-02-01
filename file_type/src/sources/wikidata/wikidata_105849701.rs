use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849701: FileFormat = FileFormat {
    id: 105_849_701,
    puid: "wikidata/105849701",
    name: "C-Worthy Form (v2.x)",
    extensions: &["cwa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x2D, 0x57, 0x6F, 0x72, 0x74, 0x68, 0x79, 0x20, 0x28, 0x52, 0x29, 0x20,
                    0x46, 0x6F, 0x72, 0x6D, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x20, 0x32, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
