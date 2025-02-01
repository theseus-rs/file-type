use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850621: FileFormat = FileFormat {
    id: 105_850_621,
    puid: "wikidata/105850621",
    name: "Multibit Blockchain Checkpoints data",
    extensions: &["checkpoints"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x45, 0x43, 0x4B, 0x50, 0x4F, 0x49, 0x4E, 0x54, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
