use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_808,
        source_type: SourceType::Wikidata,
        name: "Sealed document - content (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6F, 0x66, 0x74, 0x53, 0x45, 0x41, 0x4C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
