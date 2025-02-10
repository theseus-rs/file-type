use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_144,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD Polygons (v3.0)",
        extensions: &["ply"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x50, 0x4C, 0x59, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
