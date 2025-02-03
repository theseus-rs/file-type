use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858178: FileFormat = FileFormat {
    id: 105_858_178,
    source_type: SourceType::Wikidata,
    name: "Europa Universalis IV saved game",
    extensions: &["eu4"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x55, 0x34, 0x74, 0x78, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
