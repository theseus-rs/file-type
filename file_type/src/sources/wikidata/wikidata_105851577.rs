use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851577: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_577,
        source_type: SourceType::Wikidata,
        name: "DVD TEXT data file",
        extensions: &["txtdt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x56, 0x44, 0x5F, 0x54, 0x45, 0x58, 0x54, 0x5F, 0x44, 0x41, 0x54,
                        0x41, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
