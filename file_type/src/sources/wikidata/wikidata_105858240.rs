use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858240: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_240,
        source_type: SourceType::Wikidata,
        name: "The Elder Scrolls V: Skyrim savegame",
        extensions: &["ess"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x45, 0x53, 0x56, 0x5F, 0x53, 0x41, 0x56, 0x45, 0x47, 0x41, 0x4D,
                        0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
