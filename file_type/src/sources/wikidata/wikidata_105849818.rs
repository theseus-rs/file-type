use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849818: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_818,
        source_type: SourceType::Wikidata,
        name: "Cyberboard Game",
        extensions: &["gam"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x41, 0x4D, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
