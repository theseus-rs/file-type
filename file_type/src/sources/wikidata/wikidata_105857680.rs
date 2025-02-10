use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857680: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_680,
        source_type: SourceType::Wikidata,
        name: "Copy On Write disk image",
        extensions: &["cow"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x45, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
