use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858854: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_854,
        source_type: SourceType::Wikidata,
        name: "Pascal Script binary",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x46, 0x50, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
