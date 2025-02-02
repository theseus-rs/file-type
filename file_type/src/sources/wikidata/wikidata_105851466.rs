use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851466: FileFormat = FileFormat {
    id: 105_851_466,
    source_type: SourceType::Wikidata,
    name: "StarWriter for MS-DOS document (generic)",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2E, 0x5C, 0x5C, 0x5C, 0x20, 0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
