use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850967: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_967,
        source_type: SourceType::Wikidata,
        name: "NeroAudio Peak file",
        extensions: &["tmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x72, 0x6F, 0x41, 0x75, 0x64, 0x69, 0x6F, 0x20, 0x2D, 0x20,
                        0x50, 0x65, 0x61, 0x6B, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
