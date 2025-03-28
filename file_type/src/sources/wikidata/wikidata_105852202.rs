use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852202: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_202,
        source_type: SourceType::Wikidata,
        name: "OpenTTD savegame",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x54, 0x54, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
