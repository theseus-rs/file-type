use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_186,
        source_type: SourceType::Wikidata,
        name: "Outlook Shortcuts",
        extensions: &["fav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x09, 0x00, 0x46, 0x00, 0x61, 0x00, 0x76, 0x00, 0x50, 0x00, 0x6C, 0x00,
                        0x61, 0x00, 0x63, 0x00, 0x65, 0x00, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
