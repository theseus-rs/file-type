use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857011: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_011,
        source_type: SourceType::Wikidata,
        name: "GEDCOM Family History (UTF-8)",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x30, 0x20, 0x48, 0x45, 0x41, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
