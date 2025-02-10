use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855346: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_346,
        source_type: SourceType::Wikidata,
        name: "Foobar 2000 Columns UI settings",
        extensions: &["fcs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA5, 0xE2, 0xD6, 0x05, 0xD5, 0x9D, 0xCA, 0x23, 0xBE, 0x5A, 0x74, 0x72,
                        0x3C, 0x6F, 0x0C, 0x75,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
