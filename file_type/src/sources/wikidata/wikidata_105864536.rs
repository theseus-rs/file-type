use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864536: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_536,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Backdrop",
        extensions: &["pbk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x50, 0x42, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
