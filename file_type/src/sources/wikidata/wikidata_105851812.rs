use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851812: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_812,
        source_type: SourceType::Wikidata,
        name: "sPlan schematic (generic)",
        extensions: &["spl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x07, 0x53, 0x50, 0x4C, 0x41, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
