use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861954: FileFormat = FileFormat {
    id: 105_861_954,
    puid: "wikidata/105861954",
    name: "MEMU Floppy image",
    extensions: &["mfloppy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC3, 0x40, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
