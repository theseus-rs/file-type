use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000677: FileFormat = FileFormat {
    id: 29_000_677,
    puid: "wikidata/29000677",
    name: "Yet Another Object Description Language",
    extensions: &["ydl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x59, 0x41, 0x4F, 0x44, 0x4C, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
