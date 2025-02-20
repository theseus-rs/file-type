use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_144,
        source_type: SourceType::Wikidata,
        name: "Kt Interchange File Format compressed bitmap",
        extensions: &["kif", "kiff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
