use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_605,
        source_type: SourceType::Wikidata,
        name: "the Word Encrypted Bible Text Module",
        extensions: &["ontx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x57, 0x45, 0x4E, 0x43, 0x42, 0x4D, 0x4F, 0x44, 0x01, 0x01, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
