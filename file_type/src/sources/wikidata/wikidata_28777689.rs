use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28777689: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_689,
        source_type: SourceType::Wikidata,
        name: "Mosaic hotlists",
        extensions: &["hot"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x6F, 0x74, 0x6C, 0x69, 0x73, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
