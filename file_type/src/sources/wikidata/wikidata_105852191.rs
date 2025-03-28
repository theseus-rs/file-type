use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852191: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_191,
        source_type: SourceType::Wikidata,
        name: "Spartan spinput format",
        extensions: &["spinput"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x4F, 0x50, 0x54, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
