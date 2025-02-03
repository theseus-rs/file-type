use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850721: FileFormat = FileFormat {
    id: 105_850_721,
    source_type: SourceType::Wikidata,
    name: "Kidspiration file",
    extensions: &["kid"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x24, 0x24, 0x24, 0x24, 0x29, 0x28, 0x63, 0x29, 0x32, 0x30, 0x30,
                    0x30, 0x20, 0x49, 0x6E, 0x73, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x2C, 0x20, 0x49, 0x6E,
                    0x63, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
