use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866208: FileFormat = FileFormat {
    id: 105_866_208,
    source_type: SourceType::Wikidata,
    name: "COSMI 3-D Floorplan Designer Plan (v2.0)",
    extensions: &["pln"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x20, 0x33, 0x44, 0x2D, 0x46, 0x6C, 0x6F, 0x6F, 0x72, 0x50, 0x6C,
                    0x61, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x2D, 0x2D, 0x0D, 0x0A, 0x0D,
                    0x0A, 0x7C, 0x20, 0x31, 0x20, 0x32, 0x2E, 0x30, 0x20, 0x7E, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E, 0x0D, 0x0A, 0x0D, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
