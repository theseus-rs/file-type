use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866499: FileFormat = FileFormat {
    id: 105_866_499,
    puid: "wikidata/105866499",
    name: "Messenger Plus! Backup Configuration",
    extensions: &["pld"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4C, 0x55, 0x53, 0x5F, 0x50, 0x52, 0x45, 0x46, 0x5F, 0x50, 0x41, 0x43,
                    0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
