use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857846: FileFormat = FileFormat {
    id: 105_857_846,
    source_type: SourceType::Wikidata,
    name: "OpenSceneGraph native binary format",
    extensions: &["ive"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0x03, 0x02, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
