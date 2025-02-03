use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863367: FileFormat = FileFormat {
    id: 105_863_367,
    source_type: SourceType::Wikidata,
    name: "Earth And Beyond game data archive",
    extensions: &["mix"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x49, 0x58, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
