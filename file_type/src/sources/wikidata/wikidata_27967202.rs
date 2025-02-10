use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967202: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_202,
        source_type: SourceType::Wikidata,
        name: "Nerdtracker II module",
        extensions: &["ned"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x45, 0x44, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
