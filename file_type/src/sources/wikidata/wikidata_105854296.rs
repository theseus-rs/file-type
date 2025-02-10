use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854296: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_296,
        source_type: SourceType::Wikidata,
        name: "MPEG-2 LC-AAC Audio",
        extensions: &["aac"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xF9])],
                },
            }],
        }],
        related_formats: &[],
    },
};
