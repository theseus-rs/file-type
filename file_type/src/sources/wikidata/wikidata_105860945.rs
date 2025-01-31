use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860945: FileFormat = FileFormat {
    id: 105_860_945,
    puid: "wikidata/105860945",
    name: "Lyric file (with ID tags)",
    extensions: &["lrc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x74, 0x69, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
