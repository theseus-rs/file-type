use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867173: FileFormat = FileFormat {
    id: 105_867_173,
    puid: "wikidata/105867173",
    name: "the Word Encrypted New Testament Text Module",
    extensions: &["ntx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x57, 0x45, 0x4E, 0x43, 0x42, 0x4D, 0x4F, 0x44, 0x01, 0x01, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
