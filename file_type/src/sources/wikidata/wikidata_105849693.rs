use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849693: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_693,
        source_type: SourceType::Wikidata,
        name: "Motion Capture File Format",
        extensions: &["csm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
