use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855663: FileFormat = FileFormat {
    id: 105_855_663,
    source_type: SourceType::Wikidata,
    name: "OneNote table of contents",
    extensions: &["onetoc2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xA1, 0x2F, 0xFF, 0x43, 0xD9, 0xEF, 0x76, 0x4C, 0x9E, 0xE2, 0x10, 0xEA, 0x57,
                    0x22, 0x76, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
