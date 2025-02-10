use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_111743198: FileType = FileType {
    file_format: &FileFormat {
        id: 111_743_198,
        source_type: SourceType::Wikidata,
        name: "Sony Virtual Expander File",
        extensions: &["vem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x56, 0x20, 0x56, 0x45, 0x20, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
