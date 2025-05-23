use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866160: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_160,
        source_type: SourceType::Wikidata,
        name: "PageDraw document",
        extensions: &["pdx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                        0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x33, 0x2E, 0x30, 0x0D,
                        0x0A, 0x25, 0x25, 0x43, 0x72, 0x65, 0x61, 0x74, 0x6F, 0x72, 0x3A, 0x20,
                        0x50, 0x61, 0x67, 0x65, 0x44, 0x72, 0x61, 0x77, 0x2C, 0x20, 0x56, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
