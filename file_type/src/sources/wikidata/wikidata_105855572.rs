use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855572: FileFormat = FileFormat {
    id: 105_855_572,
    source_type: SourceType::Wikidata,
    name: "OpenColorIO profile",
    extensions: &["ocio"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6F, 0x63, 0x69, 0x6F, 0x5F, 0x70, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5F,
                    0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
