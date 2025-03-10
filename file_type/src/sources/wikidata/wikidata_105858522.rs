use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858522: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_522,
        source_type: SourceType::Wikidata,
        name: "binpatch format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFA, 0xDE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
