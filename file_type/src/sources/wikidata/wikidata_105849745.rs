use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849745: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_745,
        source_type: SourceType::Wikidata,
        name: "Atari Cartridge",
        extensions: &["car"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x52, 0x54, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
