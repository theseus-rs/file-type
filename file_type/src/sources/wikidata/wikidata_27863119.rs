use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863119: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_119,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Drawing, version 2.5",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0x10, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
