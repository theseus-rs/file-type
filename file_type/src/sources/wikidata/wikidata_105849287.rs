use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_287,
        source_type: SourceType::Wikidata,
        name: "YACE64 3D layout",
        extensions: &["y3d"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6F, 0x75, 0x6E, 0x74, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
