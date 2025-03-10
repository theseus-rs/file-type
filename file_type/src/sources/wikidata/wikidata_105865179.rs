use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865179: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_179,
        source_type: SourceType::Wikidata,
        name: "Python Pickle serialized data (v2)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
