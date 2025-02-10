use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857057: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_057,
        source_type: SourceType::Wikidata,
        name: "Genbox Family History dictionary",
        extensions: &["gdt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x42, 0x4F, 0x58, 0x44, 0x49, 0x43, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
