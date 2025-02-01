use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850590: FileFormat = FileFormat {
    id: 105_850_590,
    puid: "wikidata/105850590",
    name: "MacStitch/WinStitch design",
    extensions: &["chart"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x41, 0x6E, 0x20, 0x55, 0x72, 0x73, 0x61, 0x20, 0x53, 0x6F, 0x66,
                    0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2C, 0x01, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
