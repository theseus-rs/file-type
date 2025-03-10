use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858427: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_427,
        source_type: SourceType::Wikidata,
        name: "Mac OS X Mach-O 32bit Intel executable",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCE, 0xFA, 0xED, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
