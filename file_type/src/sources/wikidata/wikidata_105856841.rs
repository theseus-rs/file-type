use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856841: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_841,
        source_type: SourceType::Wikidata,
        name: "GPU Friendly Graphics format",
        extensions: &["gfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x46, 0x47, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
