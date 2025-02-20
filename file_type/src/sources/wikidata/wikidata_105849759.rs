use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849759: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_759,
        source_type: SourceType::Wikidata,
        name: "KGen Configuration",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x47, 0x45, 0x4E, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
