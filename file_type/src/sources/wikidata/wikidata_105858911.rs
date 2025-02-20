use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_911,
        source_type: SourceType::Wikidata,
        name: "raw Group 3 FAX bitmap",
        extensions: &["g3"],
        media_types: &["image/g3fax"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x14])],
                },
            }],
        }],
        related_formats: &[],
    },
};
