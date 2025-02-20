use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860507: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_507,
        source_type: SourceType::Wikidata,
        name: "Ren'Py Archive (v3)",
        extensions: &["rpa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x50, 0x41, 0x2D, 0x33, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
