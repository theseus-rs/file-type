use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856489: FileFormat = FileFormat {
    id: 105_856_489,
    source_type: SourceType::Wikidata,
    name: "Hyper File database",
    extensions: &["fic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x53, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00, 0x07, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
