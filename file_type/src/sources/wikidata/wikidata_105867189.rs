use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867189: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_189,
        source_type: SourceType::Wikidata,
        name: "mcrypt encrypted (v2.2)",
        extensions: &["nc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x6D, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
