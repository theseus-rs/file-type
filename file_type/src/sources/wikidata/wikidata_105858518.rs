use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858518: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_518,
        source_type: SourceType::Wikidata,
        name: "GEM bitmap (v2)",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
