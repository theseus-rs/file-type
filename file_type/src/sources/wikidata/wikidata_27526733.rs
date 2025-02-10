use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27526733: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_733,
        source_type: SourceType::Wikidata,
        name: "Graphics Interchange Format, version 87a",
        extensions: &["gif"],
        media_types: &["image/gif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x37, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
