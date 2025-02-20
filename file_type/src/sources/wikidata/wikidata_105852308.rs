use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_308,
        source_type: SourceType::Wikidata,
        name: "SLZ compressed data",
        extensions: &["slz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4C, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
