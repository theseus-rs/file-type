use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862766: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_766,
        source_type: SourceType::Wikidata,
        name: "Vic-Tracker module",
        extensions: &["vt"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x33, 0x54, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
