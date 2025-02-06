use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849750: FileFormat = FileFormat {
    id: 105_849_750,
    source_type: SourceType::Wikidata,
    name: "Professor Layton game data package",
    extensions: &["cpck"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x50, 0x43, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
