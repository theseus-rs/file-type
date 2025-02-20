use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_720,
        source_type: SourceType::Wikidata,
        name: "Organya 2 module",
        extensions: &["org"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x72, 0x67, 0x2D, 0x30, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
