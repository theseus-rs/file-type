use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_391,
        source_type: SourceType::Wikidata,
        name: "Macromedia Director Java Resource - Video",
        extensions: &["djr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x43, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
