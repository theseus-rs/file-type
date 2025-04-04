use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858850: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_850,
        source_type: SourceType::Wikidata,
        name: "PC Paint/Pictor bitmap",
        extensions: &["clp", "pic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x12])],
                },
            }],
        }],
        related_formats: &[],
    },
};
