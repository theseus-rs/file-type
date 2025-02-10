use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857231: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_231,
        source_type: SourceType::Wikidata,
        name: "Husqvarna Viking embroidery format",
        extensions: &["hus"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0xAF, 0xC8, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
