use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860876: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_876,
        source_type: SourceType::Wikidata,
        name: "RAGE Package Format (GTA IV Audio Midnight Club: LA)",
        extensions: &["rpf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x46, 0x50, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
