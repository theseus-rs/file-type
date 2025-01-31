use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855879: FileFormat = FileFormat {
    id: 105_855_879,
    puid: "wikidata/105855879",
    name: "Case Closed game data archive",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x49, 0x53, 0x26, 0x4C, 0x5A, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
