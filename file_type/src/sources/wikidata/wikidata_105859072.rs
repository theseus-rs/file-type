use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859072: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_072,
        source_type: SourceType::Wikidata,
        name: "AVG update package",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x5A, 0x20, 0x41, 0x56, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
