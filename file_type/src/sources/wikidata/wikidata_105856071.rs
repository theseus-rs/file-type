use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856071: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_071,
        source_type: SourceType::Wikidata,
        name: "OrangeCD theme",
        extensions: &["dax"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
