use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_020,
        source_type: SourceType::Wikidata,
        name: "CXF Vector Map Format (v2.x)",
        extensions: &["cxf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x58, 0x32, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
