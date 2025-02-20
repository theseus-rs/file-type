use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856512: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_512,
        source_type: SourceType::Wikidata,
        name: "PAM Development game data archive",
        extensions: &["wad"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x41, 0x4D, 0x5F, 0x50, 0x41, 0x4B, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
