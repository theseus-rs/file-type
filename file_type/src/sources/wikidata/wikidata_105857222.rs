use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857222: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_222,
        source_type: SourceType::Wikidata,
        name: "Hidden Agenda save game",
        extensions: &["ha"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x60, 0xD7, 0x25, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
