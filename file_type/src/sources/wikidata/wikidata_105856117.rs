use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_117,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing eXchange Format (var.1)",
        extensions: &["dxf"],
        media_types: &["image/vnd.dxf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x30, 0x0D, 0x0A, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
