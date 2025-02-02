use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865418: FileFormat = FileFormat {
    id: 105_865_418,
    source_type: SourceType::Wikidata,
    name: "Starbound game data archive",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x42, 0x41, 0x73, 0x73, 0x65, 0x74, 0x36,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
