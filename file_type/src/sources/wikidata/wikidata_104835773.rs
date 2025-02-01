use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104835773: FileFormat = FileFormat {
    id: 104_835_773,
    puid: "wikidata/104835773",
    name: "Sample Vision Format",
    extensions: &["smp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4F, 0x55, 0x4E, 0x44, 0x20, 0x53, 0x41, 0x4D, 0x50, 0x4C, 0x45, 0x20,
                    0x44, 0x41, 0x54, 0x41, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
