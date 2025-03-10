use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864724: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_724,
        source_type: SourceType::Wikidata,
        name: "Python Pickle serialized data (v3)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
