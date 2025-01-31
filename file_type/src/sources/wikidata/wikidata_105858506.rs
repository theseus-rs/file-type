use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858506: FileFormat = FileFormat {
    id: 105_858_506,
    puid: "wikidata/105858506",
    name: "Radiance High Dynamic Range bitmap (small hdr)",
    extensions: &["hdr", "rgbe"],
    media_types: &["image/vnd.radiance", "image/vnd.radiance"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x3F, 0x52, 0x47, 0x42, 0x45])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x3F, 0x52, 0x47, 0x42, 0x45])],
                },
            }],
        },
    ],
    related_formats: &[],
};
