use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851446: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_446,
        source_type: SourceType::Wikidata,
        name: "QuickTime Text subtitles",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x51, 0x54, 0x74, 0x65, 0x78, 0x74, 0x7D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
