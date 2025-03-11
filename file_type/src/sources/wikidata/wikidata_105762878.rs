use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762878: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_878,
        source_type: SourceType::Wikidata,
        name: "XNA Framework Content Pipeline Binary (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x4E, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
