use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858746: FileFormat = FileFormat {
    id: 105_858_746,
    source_type: SourceType::Wikidata,
    name: "Surf's Up game data archive",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x42, 0x32, 0x02, 0x00, 0x20, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
