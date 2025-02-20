use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849931: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_931,
        source_type: SourceType::Wikidata,
        name: "Cyberboard Gamebox",
        extensions: &["gbx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x42, 0x4F, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
