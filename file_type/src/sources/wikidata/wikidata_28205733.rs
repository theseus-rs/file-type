use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205733: FileFormat = FileFormat {
    id: 28_205_733,
    puid: "wikidata/28205733",
    name: "Award BIOS logo, version 1",
    extensions: &["bmp", "epa"],
    media_types: &["image/x-award-bioslogo", "image/x-award-bioslogo"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x09])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x11, 0x09])],
                },
            }],
        },
    ],
    related_formats: &[],
};
