use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856960: FileFormat = FileFormat {
    id: 105_856_960,
    source_type: SourceType::Wikidata,
    name: "American Conquest game data archive",
    extensions: &["gp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x50, 0x41, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
