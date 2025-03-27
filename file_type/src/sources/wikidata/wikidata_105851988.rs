use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851988: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_988,
        source_type: SourceType::Wikidata,
        name: "Ani ST Script",
        extensions: &["scr", "str"],
        media_types: &["image/x-ani-st", "text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
