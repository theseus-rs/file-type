use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851689: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_689,
        source_type: SourceType::Wikidata,
        name: "SuperTux Font",
        extensions: &["stf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x73, 0x75, 0x70, 0x65, 0x72, 0x74, 0x75, 0x78, 0x2D, 0x66, 0x6F,
                        0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
