use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858332: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_332,
        source_type: SourceType::Wikidata,
        name: "Disk IMage Archiver self-extracting disk image",
        extensions: &["exe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x5A, 0x2F, 0x00, 0x37, 0x00, 0x00, 0x00, 0x02, 0x00, 0x94, 0x03,
                        0xFF, 0xFF, 0x31, 0x0A, 0x80, 0x00, 0x00, 0x00, 0x0E, 0x00, 0xA6, 0x06,
                        0x1C, 0x00, 0x00, 0x00, 0x44, 0x49, 0x4D, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
