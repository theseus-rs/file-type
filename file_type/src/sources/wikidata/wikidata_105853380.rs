use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_380,
        source_type: SourceType::Wikidata,
        name: "Visual SourceSafe control file (var 1)",
        extensions: &["scc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x43, 0x20, 0x3D, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69,
                        0x73, 0x20, 0x61, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
