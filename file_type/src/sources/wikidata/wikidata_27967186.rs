use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967186: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_186,
        source_type: SourceType::Wikidata,
        name: "FunkTracker module",
        extensions: &["fnk"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x75, 0x6E, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
