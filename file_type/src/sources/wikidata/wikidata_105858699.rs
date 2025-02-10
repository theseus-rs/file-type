use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_699,
        source_type: SourceType::Wikidata,
        name: "Variant Call Format (binary) (v1)",
        extensions: &["bcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
