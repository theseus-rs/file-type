use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762744: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_744,
        source_type: SourceType::Wikidata,
        name: "X Millennium OPM log",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x31, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
