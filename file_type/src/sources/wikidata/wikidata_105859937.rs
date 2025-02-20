use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859937: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_937,
        source_type: SourceType::Wikidata,
        name: "WAP Bookmark info",
        extensions: &["vbm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x3A, 0x56, 0x42, 0x4B, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
