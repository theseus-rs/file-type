use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979314: FileFormat = FileFormat {
    id: 27_979_314,
    source_type: SourceType::Wikidata,
    name: "Adobe Swatch Exchange",
    extensions: &["ase", "asef"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x53, 0x45, 0x46])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x53, 0x45, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
