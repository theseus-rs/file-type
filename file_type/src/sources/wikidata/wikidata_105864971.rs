use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864971: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_971,
        source_type: SourceType::Wikidata,
        name: "PGP private key block",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x50,
                        0x47, 0x50, 0x20, 0x50, 0x52, 0x49, 0x56, 0x41, 0x54, 0x45, 0x20, 0x4B,
                        0x45, 0x59, 0x20, 0x42, 0x4C, 0x4F, 0x43, 0x4B, 0x2D, 0x2D, 0x2D, 0x2D,
                        0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
