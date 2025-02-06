use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852911: FileFormat = FileFormat {
    id: 105_852_911,
    source_type: SourceType::Wikidata,
    name: "Postal game data Archive",
    extensions: &["sak"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x4B, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
