use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857021: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_021,
        source_type: SourceType::Wikidata,
        name: "MegaZeux General Digital Music",
        extensions: &["gdm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x44, 0x4D, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
