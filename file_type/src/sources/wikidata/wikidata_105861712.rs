use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861712: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_712,
        source_type: SourceType::Wikidata,
        name: "Dynamix Music data container",
        extensions: &["mus"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
