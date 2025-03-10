use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856662: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_662,
        source_type: SourceType::Wikidata,
        name: "UHBC compressed",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x48, 0x42, 0x10, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
