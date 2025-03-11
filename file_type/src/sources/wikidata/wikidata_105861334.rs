use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861334: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_334,
        source_type: SourceType::Wikidata,
        name: "Generic Lionhead Studios game data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x69, 0x4F, 0x6E, 0x48, 0x65, 0x41, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
