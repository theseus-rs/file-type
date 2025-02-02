use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855306: FileFormat = FileFormat {
    id: 105_855_306,
    source_type: SourceType::Wikidata,
    name: "Font Specifications",
    extensions: &["fontspec"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0x66, 0x6F, 0x6E, 0x74, 0x66,
                    0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x5B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
