use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858772: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_772,
        source_type: SourceType::Wikidata,
        name: "Synu bitmap",
        extensions: &["syn", "synu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x6D, 0x61, 0x67, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
