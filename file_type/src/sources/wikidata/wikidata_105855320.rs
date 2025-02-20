use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855320: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_320,
        source_type: SourceType::Wikidata,
        name: "FlowJo Mac Workspace",
        extensions: &["jo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x6C, 0x6F, 0x77, 0x4A, 0x6F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
