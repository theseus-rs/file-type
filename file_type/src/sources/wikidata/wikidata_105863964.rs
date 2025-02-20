use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863964: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_964,
        source_type: SourceType::Wikidata,
        name: "RapidFile Memo",
        extensions: &["mmo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0xA2, 0x64, 0xB0])],
                },
            }],
        }],
        related_formats: &[],
    },
};
