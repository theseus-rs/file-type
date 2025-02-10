use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862118: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_118,
        source_type: SourceType::Wikidata,
        name: "Audio Interface Library 3 Music/MIDI driver",
        extensions: &["mdi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x49, 0x4C, 0x33, 0x4D, 0x44, 0x49, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
