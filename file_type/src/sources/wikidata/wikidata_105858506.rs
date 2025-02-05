use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858506: FileFormat = FileFormat {
    id: 105_858_506,
    source_type: SourceType::Wikidata,
    name: "Radiance High Dynamic Range bitmap (small hdr)",
    extensions: &["hdr", "rgbe"],
    media_types: &["image/vnd.radiance"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x3F, 0x52, 0x47, 0x42, 0x45])],
                },
            }],
        },
        Signature {
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
