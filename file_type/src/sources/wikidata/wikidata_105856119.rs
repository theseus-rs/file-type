use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856119: FileFormat = FileFormat {
    id: 105_856_119,
    source_type: SourceType::Wikidata,
    name: "dockzip format",
    extensions: &["doczip"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xE8, 0x04, 0x00, 0x00, 0x2E, 0x64, 0x6F, 0x63, 0x6B, 0x7A, 0x69, 0x70, 0x20,
                    0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
