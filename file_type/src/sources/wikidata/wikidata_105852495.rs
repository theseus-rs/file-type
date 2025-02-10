use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852495: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_495,
        source_type: SourceType::Wikidata,
        name: "SEG-2 data",
        extensions: &["seg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
