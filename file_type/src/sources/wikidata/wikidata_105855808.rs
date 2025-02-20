use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_808,
        source_type: SourceType::Wikidata,
        name: "1ST Word Plus Document",
        extensions: &["doc", "st"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
