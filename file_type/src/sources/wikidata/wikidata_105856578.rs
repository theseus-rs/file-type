use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856578: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_578,
        source_type: SourceType::Wikidata,
        name: "DoomRL WAD resource",
        extensions: &["wad"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x08, 0x56, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x30, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
