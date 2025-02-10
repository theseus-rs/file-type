use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859959: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_959,
        source_type: SourceType::Wikidata,
        name: "VVV Virtual Volume View database",
        extensions: &["vvv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x39, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
