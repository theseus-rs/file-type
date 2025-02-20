use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865656: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_656,
        source_type: SourceType::Wikidata,
        name: "Garmin PCX5 track file",
        extensions: &["trk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x20, 0x20, 0x53, 0x4F, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x20,
                        0x4E, 0x41, 0x4D, 0x45, 0x20, 0x26, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49,
                        0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
