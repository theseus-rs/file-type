use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762813: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_813,
        source_type: SourceType::Wikidata,
        name: "XaoS Position File",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x50, 0x6F, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
