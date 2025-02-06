use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866224: FileFormat = FileFormat {
    id: 105_866_224,
    source_type: SourceType::Wikidata,
    name: "PageWunder document",
    extensions: &["pwf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x41, 0x50, 0x4D, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x3C, 0x44, 0x6F, 0x63,
                    0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x53, 0x65, 0x74, 0x75, 0x70, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
