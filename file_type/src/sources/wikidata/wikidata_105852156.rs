use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852156: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_156,
        source_type: SourceType::Wikidata,
        name: "DemoManiac Script",
        extensions: &["script"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4D, 0x56, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
