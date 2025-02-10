use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849756: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_756,
        source_type: SourceType::Wikidata,
        name: "ColorSchemer Studio Color Scheme",
        extensions: &["cs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x53, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
