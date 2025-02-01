use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_375296: FileFormat = FileFormat {
    id: 375_296,
    puid: "wikidata/375296",
    name: "ICC profile",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile", "application/vnd.iccprofile"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x63, 0x73, 0x70])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x63, 0x73, 0x70])],
                },
            }],
        },
    ],
    related_formats: &[],
};
