use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858344: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_344,
        source_type: SourceType::Wikidata,
        name: "Eclipse text",
        extensions: &["ecl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x63, 0x6C, 0x69, 0x70, 0x73, 0x65, 0x20, 0x74, 0x65, 0x78, 0x74,
                        0x2E, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
