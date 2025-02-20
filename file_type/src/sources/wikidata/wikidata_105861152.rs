use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861152: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_152,
        source_type: SourceType::Wikidata,
        name: "Mythos Software LIB game data container",
        extensions: &["lib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x49, 0x42, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
