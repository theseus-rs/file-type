use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27526739: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_739,
        source_type: SourceType::Wikidata,
        name: "Graphics Interchange Format, version 89a",
        extensions: &["gif"],
        media_types: &["image/gif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
