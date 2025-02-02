use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863123: FileFormat = FileFormat {
    id: 105_863_123,
    source_type: SourceType::Wikidata,
    name: "SawTeeth module (text format)",
    extensions: &["st"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x57, 0x54, 0x54, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
