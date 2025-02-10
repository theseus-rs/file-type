use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849278: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_278,
        source_type: SourceType::Wikidata,
        name: "ST-Sound YM chiptune (generic)",
        extensions: &["ym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
