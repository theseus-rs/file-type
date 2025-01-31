use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864365: FileFormat = FileFormat {
    id: 105_864_365,
    puid: "wikidata/105864365",
    name: "PCVIC VIC-20 emulator saved-session",
    extensions: &["pcv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x56, 0x49, 0x43, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20,
                    0x73, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F, 0x74, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
