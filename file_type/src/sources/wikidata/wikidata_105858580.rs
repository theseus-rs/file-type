use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858580: FileFormat = FileFormat {
    id: 105_858_580,
    puid: "wikidata/105858580",
    name: "VIPS bitmap",
    extensions: &["v", "vips"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0xF2, 0xA6, 0xB6])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0xF2, 0xA6, 0xB6])],
                },
            }],
        },
    ],
    related_formats: &[],
};
