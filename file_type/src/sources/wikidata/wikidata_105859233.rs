use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859233: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_233,
        source_type: SourceType::Wikidata,
        name: "Variant Call Format (binary) (v2.1)",
        extensions: &["bcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x02, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
