use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858979: FileFormat = FileFormat {
    id: 105_858_979,
    puid: "wikidata/105858979",
    name: "Tagged Image File Format Bitmap (big endian)",
    extensions: &["tif", "tiff"],
    media_types: &["image/tiff", "image/tiff"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
