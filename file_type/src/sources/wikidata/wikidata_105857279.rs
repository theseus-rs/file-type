use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857279: FileFormat = FileFormat {
    id: 105_857_279,
    source_type: SourceType::Wikidata,
    name: "Windows NT Registry Hive (generic)",
    extensions: &["dat", "hiv"],
    media_types: &["application/x-ms-registry"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x65, 0x67, 0x66])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x65, 0x67, 0x66])],
                },
            }],
        },
    ],
    related_formats: &[],
};
