use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975811: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_811,
        source_type: SourceType::Wikidata,
        name: "GOCAD ASCII data format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4F, 0x43, 0x41, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
