use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856656: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_656,
        source_type: SourceType::Wikidata,
        name: "id Software's DOOM Patch-WAD",
        extensions: &["wad"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x57, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
