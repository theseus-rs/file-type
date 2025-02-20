use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858495: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_495,
        source_type: SourceType::Wikidata,
        name: "ScriptBasic Binary File Format (64bit)",
        extensions: &["bbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x38, 0x42, 0x41, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
