use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855989: FileFormat = FileFormat {
    id: 105_855_989,
    source_type: SourceType::Wikidata,
    name: "Device Tree Blob/Overlay",
    extensions: &["dtb", "dtbo"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD0, 0x0D, 0xFE, 0xED, 0x00, 0x00])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD0, 0x0D, 0xFE, 0xED, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
