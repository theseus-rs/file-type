use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_42591: FileType = FileType {
    file_format: &FileFormat {
        id: 42_591,
        source_type: SourceType::Wikidata,
        name: "MP3",
        extensions: &["mp3"],
        media_types: &["audio/mpeg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
