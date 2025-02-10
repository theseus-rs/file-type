use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854909: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_909,
        source_type: SourceType::Wikidata,
        name: "AceNotes PIM data",
        extensions: &["an2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x63, 0x65, 0x4E, 0x6F, 0x74, 0x65, 0x73, 0x32, 0x30, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
