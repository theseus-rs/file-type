use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859780: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_780,
        source_type: SourceType::Wikidata,
        name: "Maui Runtime Environment application (Zlib packed)",
        extensions: &["vxp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0xDA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
