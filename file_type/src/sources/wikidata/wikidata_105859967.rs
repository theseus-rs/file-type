use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859967: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_967,
        source_type: SourceType::Wikidata,
        name: "Visual Boy Advance movie capture",
        extensions: &["vbm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x42, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
