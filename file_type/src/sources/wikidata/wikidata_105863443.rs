use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863443: FileFormat = FileFormat {
    id: 105_863_443,
    puid: "wikidata/105863443",
    name: "Word for the Macintosh/Write for Atari ST document (v1.0)",
    extensions: &["doc", "mcw"],
    media_types: &["application/msword", "application/msword"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x32, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x32, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
