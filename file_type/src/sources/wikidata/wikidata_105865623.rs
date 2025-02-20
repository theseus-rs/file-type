use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865623: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_623,
        source_type: SourceType::Wikidata,
        name: "Professional Sound Artists module",
        extensions: &["psa"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x41, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
