use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855828: FileFormat = FileFormat {
    id: 105_855_828,
    source_type: SourceType::Wikidata,
    name: "Norton Textra Writer Document (v7.x)",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0xFD, 0xFF, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x1A, 0x00,
                    0x00, 0x07, 0x00, 0x64, 0x00, 0x3A, 0x01, 0x00, 0x00, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
