use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_087,
        source_type: SourceType::Wikidata,
        name: "NorthStar disk Image (NS DOS)",
        extensions: &["nsi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x4F, 0x53, 0x20, 0x20, 0x20, 0x20, 0x20, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
