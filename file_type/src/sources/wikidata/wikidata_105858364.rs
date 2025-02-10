use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_364,
        source_type: SourceType::Wikidata,
        name: "ETABS model",
        extensions: &["edb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x54, 0x41, 0x42, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
