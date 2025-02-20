use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855757: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_757,
        source_type: SourceType::Wikidata,
        name: "Micrografx Designer Drawing (v9.0)",
        extensions: &["dsf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
