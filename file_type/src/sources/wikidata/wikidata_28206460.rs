use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206460: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_460,
        source_type: SourceType::Wikidata,
        name: "KiSS color file",
        extensions: &["kcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x69, 0x53, 0x53, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
