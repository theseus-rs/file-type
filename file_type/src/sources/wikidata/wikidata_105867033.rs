use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_033,
        source_type: SourceType::Wikidata,
        name: "Project Nomads Texture",
        extensions: &["ntx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0x58, 0x54, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
