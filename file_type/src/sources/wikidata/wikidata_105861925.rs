use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861925: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_925,
        source_type: SourceType::Wikidata,
        name: "Windows Easy Transfer migration data",
        extensions: &["mig"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x67, 0x69, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
