use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_391,
        source_type: SourceType::Wikidata,
        name: "Motorola phone skin info",
        extensions: &["ski"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4B, 0x49, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
