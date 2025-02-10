use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858382: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_382,
        source_type: SourceType::Wikidata,
        name: "HyperVision EMF ASCII Format",
        extensions: &["emf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4D, 0x46, 0x31, 0x2E, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20, 0x23,
                        0x23, 0x20, 0x48, 0x79, 0x70, 0x65, 0x72, 0x56, 0x69, 0x73, 0x69, 0x6F,
                        0x6E, 0x20, 0x45, 0x4D, 0x46, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20,
                        0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
