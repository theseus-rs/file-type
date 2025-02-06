use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857423: FileFormat = FileFormat {
    id: 105_857_423,
    source_type: SourceType::Wikidata,
    name: "Compass and Ruler Job",
    extensions: &["job"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69,
                    0x6E, 0x67, 0x3D, 0x22, 0x75, 0x74, 0x66, 0x2D, 0x38, 0x22, 0x3F, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
