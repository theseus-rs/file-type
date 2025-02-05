use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857466: FileFormat = FileFormat {
    id: 105_857_466,
    source_type: SourceType::Wikidata,
    name: "360desktop 360-degree Desktop image",
    extensions: &["360"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xED, 0x33, 0x36, 0x30, 0x64, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x0D, 0x0A,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
