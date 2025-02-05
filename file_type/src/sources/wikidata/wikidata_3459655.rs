use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3459655: FileFormat = FileFormat {
    id: 3_459_655,
    source_type: SourceType::Wikidata,
    name: "StuffIt X archive",
    extensions: &["sitx"],
    media_types: &["application/x-sitx", "application/x-stuffitx"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x21,
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
                        0x53, 0x74, 0x75, 0x66, 0x66, 0x49, 0x74, 0x21,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
