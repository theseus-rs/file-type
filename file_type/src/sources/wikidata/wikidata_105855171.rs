use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855171: FileFormat = FileFormat {
    id: 105_855_171,
    puid: "wikidata/105855171",
    name: "BitLocker Disk Encryption volume",
    extensions: &["fve"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0x58, 0x90, 0x2D, 0x46, 0x56, 0x45, 0x2D, 0x46, 0x53, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
