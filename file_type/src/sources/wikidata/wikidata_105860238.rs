use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860238: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_238,
        source_type: SourceType::Wikidata,
        name: "Reflections scene/project",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x43, 0x00, 0x4F, 0x00, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
