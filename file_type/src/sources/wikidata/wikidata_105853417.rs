use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853417: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_417,
        source_type: SourceType::Wikidata,
        name: "Spectral Data file",
        extensions: &["spa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x61, 0x6C, 0x20, 0x44, 0x61, 0x74,
                        0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
