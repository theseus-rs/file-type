use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857192: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_192,
        source_type: SourceType::Wikidata,
        name: "HEC-HMS Metereologic model configuration",
        extensions: &["met"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x65, 0x74, 0x65, 0x6F, 0x72, 0x6F, 0x6C, 0x6F, 0x67, 0x79, 0x3A,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
