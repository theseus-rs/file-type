use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857246: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_246,
        source_type: SourceType::Wikidata,
        name: "MaxonMAGIC Sound sample (v1.0)",
        extensions: &["hsn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x53, 0x4E, 0x44, 0x31, 0x2E, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
