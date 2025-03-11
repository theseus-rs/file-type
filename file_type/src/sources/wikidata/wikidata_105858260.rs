use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858260: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_260,
        source_type: SourceType::Wikidata,
        name: "Mac OS X Universal Binary executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBA, 0xBE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
