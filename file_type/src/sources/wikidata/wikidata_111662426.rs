use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_111662426: FileType = FileType {
    file_format: &FileFormat {
        id: 111_662_426,
        source_type: SourceType::Wikidata,
        name: "MGI PhotoSuite 1.0/8.0 Album File",
        extensions: &["ctf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4A, 0x2B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
