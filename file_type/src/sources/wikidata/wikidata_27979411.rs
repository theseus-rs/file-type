use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979411: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_411,
        source_type: SourceType::Wikidata,
        name: "iCEDraw",
        extensions: &["idf"],
        media_types: &["application/octet-stream", "image/x-icedraw"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x31, 0x2E, 0x34, 0x00, 0x00, 0x00, 0x00, 0x4F, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
