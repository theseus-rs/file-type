use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862672: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_672,
        source_type: SourceType::Wikidata,
        name: "MBasic source",
        extensions: &["mbi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x42, 0x4D, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
