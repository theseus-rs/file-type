use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856881: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_881,
        source_type: SourceType::Wikidata,
        name: "Generator GNM output log / music",
        extensions: &["gnm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4E, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
