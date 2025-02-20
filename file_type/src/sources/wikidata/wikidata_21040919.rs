use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_21040919: FileType = FileType {
    file_format: &FileFormat {
        id: 21_040_919,
        source_type: SourceType::Wikidata,
        name: "MultiTracker format",
        extensions: &["mtm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x54, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
