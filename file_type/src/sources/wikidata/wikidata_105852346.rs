use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852346: FileFormat = FileFormat {
    id: 105_852_346,
    source_type: SourceType::Wikidata,
    name: "Ground Control II: Operation Exodus game data archive",
    extensions: &["sdf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x59, 0x53, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
