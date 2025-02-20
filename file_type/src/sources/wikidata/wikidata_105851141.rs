use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851141: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_141,
        source_type: SourceType::Wikidata,
        name: "TransCopy disk image",
        extensions: &["tc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0xA5])],
                },
            }],
        }],
        related_formats: &[],
    },
};
