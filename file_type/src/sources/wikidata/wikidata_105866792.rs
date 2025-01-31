use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866792: FileFormat = FileFormat {
    id: 105_866_792,
    puid: "wikidata/105866792",
    name: "PMD 85 emulator snapshot",
    extensions: &["psn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x4E, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
