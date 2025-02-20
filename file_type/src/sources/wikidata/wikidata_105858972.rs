use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858972: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_972,
        source_type: SourceType::Wikidata,
        name: "Compiled Erlang code (old)",
        extensions: &["beam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x42, 0x45, 0x41, 0x4D, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
