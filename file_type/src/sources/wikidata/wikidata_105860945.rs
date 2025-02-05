use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860945: FileFormat = FileFormat {
    id: 105_860_945,
    source_type: SourceType::Wikidata,
    name: "Lyric file (with ID tags)",
    extensions: &["lrc"],
    media_types: &[],
    signatures: &[Signature {
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
