use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855323: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_323,
        source_type: SourceType::Wikidata,
        name: "JB BAHN vehicle (Zoom2)",
        extensions: &["fz2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x42, 0x20, 0x42, 0x41, 0x48, 0x4E, 0x20, 0x56, 0x65, 0x68, 0x69,
                        0x63, 0x6C, 0x65, 0x20, 0x67, 0x66, 0x78, 0x20, 0x5A, 0x6F, 0x6F, 0x6D,
                        0x32, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
