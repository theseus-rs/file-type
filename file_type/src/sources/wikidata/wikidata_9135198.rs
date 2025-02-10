use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_9135198: FileType = FileType {
    file_format: &FileFormat {
        id: 9_135_198,
        source_type: SourceType::Wikidata,
        name: "Composer 669 module",
        extensions: &["669"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
