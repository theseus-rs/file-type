use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856312: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_312,
        source_type: SourceType::Wikidata,
        name: "Timeworks Publisher/Publish It! document",
        extensions: &["dtp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x50, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
