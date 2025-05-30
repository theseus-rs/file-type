use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853477: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_477,
        source_type: SourceType::Wikidata,
        name: "Zstandard compressed data",
        extensions: &["zst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB5, 0x2F, 0xFD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
