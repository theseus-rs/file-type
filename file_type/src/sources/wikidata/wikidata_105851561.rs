use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851561: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_561,
        source_type: SourceType::Wikidata,
        name: "SACD multi-channel TOC",
        extensions: &["toc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x55, 0x4C, 0x43, 0x48, 0x54, 0x4F, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
