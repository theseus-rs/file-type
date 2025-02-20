use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861118: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_118,
        source_type: SourceType::Wikidata,
        name: "Handy savestate (v2)",
        extensions: &["lss"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x53, 0x53, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
