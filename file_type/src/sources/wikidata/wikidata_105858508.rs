use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858508: FileFormat = FileFormat {
    id: 105_858_508,
    source_type: SourceType::Wikidata,
    name: "Blizzard Picture (type 1)",
    extensions: &["blp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4C, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
