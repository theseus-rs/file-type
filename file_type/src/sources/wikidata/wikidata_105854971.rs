use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854971: FileFormat = FileFormat {
    id: 105_854_971,
    puid: "wikidata/105854971",
    name: "BIX Archiver compressed archive",
    extensions: &["bix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x58, 0x30, 0xC1, 0xB8, 0x03, 0x9A, 0xF1,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
