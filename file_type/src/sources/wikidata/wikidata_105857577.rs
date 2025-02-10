use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857577: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_577,
        source_type: SourceType::Wikidata,
        name: "Actor Image snapshot (v4.1)",
        extensions: &["ima"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x10, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
