use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854995: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_995,
        source_type: SourceType::Wikidata,
        name: "askSam database",
        extensions: &["ask"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x73, 0x6B, 0x53, 0x61, 0x6D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
