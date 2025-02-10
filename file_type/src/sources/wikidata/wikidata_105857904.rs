use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857904: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_904,
        source_type: SourceType::Wikidata,
        name: "UAE Input recorder data",
        extensions: &["inp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x41, 0x45, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
