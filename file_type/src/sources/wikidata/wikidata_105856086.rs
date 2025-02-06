use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856086: FileFormat = FileFormat {
    id: 105_856_086,
    source_type: SourceType::Wikidata,
    name: "Impulse Tracker Advanced Sound Driver",
    extensions: &["drv", "mmx"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6D, 0x70, 0x75, 0x6C, 0x73, 0x65, 0x20, 0x54, 0x72, 0x61, 0x63,
                        0x6B, 0x65, 0x72, 0x20, 0x41, 0x64, 0x76, 0x61, 0x6E, 0x63, 0x65, 0x64,
                        0x20, 0x53, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x44, 0x72, 0x69, 0x76, 0x65,
                        0x72, 0x0D, 0x0A,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6D, 0x70, 0x75, 0x6C, 0x73, 0x65, 0x20, 0x54, 0x72, 0x61, 0x63,
                        0x6B, 0x65, 0x72, 0x20, 0x41, 0x64, 0x76, 0x61, 0x6E, 0x63, 0x65, 0x64,
                        0x20, 0x53, 0x6F, 0x75, 0x6E, 0x64, 0x20, 0x44, 0x72, 0x69, 0x76, 0x65,
                        0x72, 0x0D, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
