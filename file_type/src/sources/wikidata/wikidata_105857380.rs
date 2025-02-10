use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857380: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_380,
        source_type: SourceType::Wikidata,
        name: "BeamNG vehicle definition format",
        extensions: &["jbeam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0D, 0x0A, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
