use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27863132: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_132,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2000-2002",
        extensions: &["dwg"],
        media_types: &["application/x-autocad", "image/vnd.dwg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x31, 0x35])],
                },
            }],
        }],
        related_formats: &[],
    },
};
