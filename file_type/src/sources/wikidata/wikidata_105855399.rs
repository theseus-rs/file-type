use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855399: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_399,
        source_type: SourceType::Wikidata,
        name: "DemoManiac Font",
        extensions: &["font"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x6F, 0x4E, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
