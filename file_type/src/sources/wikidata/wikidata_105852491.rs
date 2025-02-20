use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852491: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_491,
        source_type: SourceType::Wikidata,
        name: "DataShow sounds/music",
        extensions: &["snd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x44, 0x61, 0x74, 0x61, 0x53, 0x68, 0x6F, 0x77,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
