use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853461: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_461,
        source_type: SourceType::Wikidata,
        name: "ZX spectrum +3 DOS data (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x4C, 0x55, 0x53, 0x33, 0x44, 0x4F, 0x53, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
