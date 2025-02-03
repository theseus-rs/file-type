use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863612: FileFormat = FileFormat {
    id: 105_863_612,
    source_type: SourceType::Wikidata,
    name: "Synthesis module",
    extensions: &["syn"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x79, 0x6E, 0x74, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
