use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857643: FileFormat = FileFormat {
    id: 105_857_643,
    source_type: SourceType::Wikidata,
    name: "CP Backup disk image",
    extensions: &["dsk", "img"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x42, 0x61, 0x63, 0x6B, 0x75, 0x70,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x42, 0x61, 0x63, 0x6B, 0x75, 0x70,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
