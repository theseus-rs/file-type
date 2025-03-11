use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849832: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_832,
        source_type: SourceType::Wikidata,
        name: "Compo filters",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x61, 0x6D, 0x65, 0x3A, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
