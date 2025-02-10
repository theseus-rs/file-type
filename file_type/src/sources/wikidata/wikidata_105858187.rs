use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858187: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_187,
        source_type: SourceType::Wikidata,
        name: "EZGUI Designer control definition",
        extensions: &["ezc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3C, 0x3C, 0x44, 0x45, 0x53, 0x43, 0x3E, 0x3E, 0x3E, 0x0D, 0x0A,
                        0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
