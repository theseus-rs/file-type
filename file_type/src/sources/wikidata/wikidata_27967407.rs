use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967407: FileFormat = FileFormat {
    id: 27_967_407,
    source_type: SourceType::Wikidata,
    name: "Surprise! Adlib Tracker version 2.0",
    extensions: &["sa2"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
