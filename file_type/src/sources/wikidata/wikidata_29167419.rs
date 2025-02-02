use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167419: FileFormat = FileFormat {
    id: 29_167_419,
    source_type: SourceType::Wikidata,
    name: "Internet Adventure Game Engine compiled game",
    extensions: &["iage"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x41, 0x47, 0x45, 0x20, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
