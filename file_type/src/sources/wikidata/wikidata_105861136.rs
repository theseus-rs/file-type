use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861136: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_136,
        source_type: SourceType::Wikidata,
        name: "LD Linker Script",
        extensions: &["lds"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x55, 0x54, 0x50, 0x55, 0x54, 0x5F, 0x46, 0x4F, 0x52, 0x4D, 0x41,
                        0x54, 0x28, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
