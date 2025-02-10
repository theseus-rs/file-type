use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_131,
        source_type: SourceType::Wikidata,
        name: "Art Of Noise 8-channel module",
        extensions: &["aon"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
