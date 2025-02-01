use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849626: FileFormat = FileFormat {
    id: 105_849_626,
    puid: "wikidata/105849626",
    name: "Super Angelo Configuration",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x55, 0x50, 0x45, 0x52, 0x20, 0x41, 0x4E, 0x47, 0x45, 0x4C, 0x4F, 0x20,
                    0x43, 0x46, 0x47, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
