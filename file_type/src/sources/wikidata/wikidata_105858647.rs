use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858647: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_647,
        source_type: SourceType::Wikidata,
        name: "Bradford Font (v2)",
        extensions: &["bf2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x46, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
