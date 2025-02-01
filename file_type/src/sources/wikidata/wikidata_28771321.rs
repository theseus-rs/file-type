use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771321: FileFormat = FileFormat {
    id: 28_771_321,
    puid: "wikidata/28771321",
    name: "Microsoft Update Standalone Package",
    extensions: &["msu", "msu"],
    media_types: &["application/vnd.ms-cab-compressed", "text/xml"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
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
                        0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
