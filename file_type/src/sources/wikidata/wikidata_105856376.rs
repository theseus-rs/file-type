use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856376: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_376,
        source_type: SourceType::Wikidata,
        name: "Digital Tracker 1.9 module",
        extensions: &["dtm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x2E, 0x54, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
