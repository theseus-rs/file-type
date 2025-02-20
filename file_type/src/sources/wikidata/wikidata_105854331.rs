use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854331: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_331,
        source_type: SourceType::Wikidata,
        name: "Art Of Noise 4-channel module",
        extensions: &["aon"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
