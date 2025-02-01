use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853163: FileFormat = FileFormat {
    id: 105_853_163,
    puid: "wikidata/105853163",
    name: "American's McGee's Alice Saved Game File",
    extensions: &["sav", "spv"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4C, 0x49, 0x43, 0x01, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00,
                        0x41, 0x6C, 0x69, 0x63, 0x65, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4C, 0x49, 0x43, 0x01, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00,
                        0x41, 0x6C, 0x69, 0x63, 0x65, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
