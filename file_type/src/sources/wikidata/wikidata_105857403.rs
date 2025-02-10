use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_403,
        source_type: SourceType::Wikidata,
        name: "LegaSuite GUI Runtime",
        extensions: &["jwr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x41, 0x47, 0x55, 0x4C, 0x4C, 0x20, 0x4A, 0x57, 0x52, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
