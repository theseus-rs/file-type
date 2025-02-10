use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858412: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_412,
        source_type: SourceType::Wikidata,
        name: "EarAche module",
        extensions: &["ea"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x41, 0x53, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
