use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852195: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_195,
        source_type: SourceType::Wikidata,
        name: "Limbo Symbol table",
        extensions: &["sbl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6C, 0x69, 0x6D, 0x62, 0x6F, 0x20, 0x2E, 0x73, 0x62, 0x6C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
