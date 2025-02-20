use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206490: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_490,
        source_type: SourceType::Wikidata,
        name: "LazPaint LZP file format",
        extensions: &["lzp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x7A, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
