use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979314: FileFormat = FileFormat {
    id: 27_979_314,
    source_type: SourceType::Wikidata,
    name: "Adobe Swatch Exchange",
    extensions: &["ase", "asef"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x53, 0x45, 0x46])],
                },
            }],
        },
        InternalSignature {
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
