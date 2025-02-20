use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853558: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_558,
        source_type: SourceType::Wikidata,
        name: "SCO compress LZH compressed data",
        extensions: &["z"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1F, 0xA0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
