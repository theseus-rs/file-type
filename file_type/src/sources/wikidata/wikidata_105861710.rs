use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_710,
        source_type: SourceType::Wikidata,
        name: "Model4D project",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x34, 0x44, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
