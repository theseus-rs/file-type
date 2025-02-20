use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867022: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_022,
        source_type: SourceType::Wikidata,
        name: "Gambit Neutral file",
        extensions: &["neu"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x43, 0x4F, 0x4E, 0x54,
                        0x52, 0x4F, 0x4C, 0x20, 0x49, 0x4E, 0x46, 0x4F, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
