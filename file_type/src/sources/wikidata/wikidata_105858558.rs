use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858558: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_558,
        source_type: SourceType::Wikidata,
        name: "Borland Language Library",
        extensions: &["bll"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4F, 0x43, 0x41, 0x4C, 0x45, 0x2E, 0x42, 0x4C, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
