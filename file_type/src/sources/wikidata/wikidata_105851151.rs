use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851151: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_151,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal Help",
        extensions: &["tph"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x55, 0x52, 0x42, 0x4F, 0x20, 0x50, 0x41, 0x53, 0x43, 0x41, 0x4C,
                        0x20, 0x48, 0x45, 0x4C, 0x50, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
