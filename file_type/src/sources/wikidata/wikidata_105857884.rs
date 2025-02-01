use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857884: FileFormat = FileFormat {
    id: 105_857_884,
    puid: "wikidata/105857884",
    name: "T98-Next Floppy Disk image (R0)",
    extensions: &["nfd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x39, 0x38, 0x46, 0x44, 0x44, 0x49, 0x4D, 0x41, 0x47, 0x45, 0x2E, 0x52,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
