use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_391,
        source_type: SourceType::Wikidata,
        name: "JustWrite document",
        extensions: &["jw", "jwt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x46, 0x46, 0x46, 0x49, 0x49, 0x49, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
