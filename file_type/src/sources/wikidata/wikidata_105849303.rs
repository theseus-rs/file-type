use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849303: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_303,
        source_type: SourceType::Wikidata,
        name: "Yahoo! Widget",
        extensions: &["widget"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x64, 0x67, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
