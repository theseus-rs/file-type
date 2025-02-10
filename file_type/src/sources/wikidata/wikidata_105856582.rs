use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856582: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_582,
        source_type: SourceType::Wikidata,
        name: "Wipeout 2097 track data",
        extensions: &["wad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x00, 0x74, 0x72, 0x61, 0x63, 0x6B, 0x2E, 0x74, 0x72, 0x76, 0x00,
                        0xA1,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
