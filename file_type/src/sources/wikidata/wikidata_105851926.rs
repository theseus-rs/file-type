use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851926: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_926,
        source_type: SourceType::Wikidata,
        name: "Spectrum Sound Tracker 1.3 chiptune",
        extensions: &["st13"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x4F, 0x55, 0x4E, 0x44, 0x20, 0x54, 0x52, 0x41, 0x43, 0x4B, 0x45,
                        0x52, 0x20, 0x76, 0x31, 0x2E, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
