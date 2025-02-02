use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866132: FileFormat = FileFormat {
    id: 105_866_132,
    source_type: SourceType::Wikidata,
    name: "World Construction Set Parameters (v2.x)",
    extensions: &["par"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x57, 0x43, 0x53, 0x50, 0x41, 0x52, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
