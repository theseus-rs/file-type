use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857625: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_625,
        source_type: SourceType::Wikidata,
        name: "ISo Zipped format",
        extensions: &["isz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x73, 0x5A, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
