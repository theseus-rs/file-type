use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857524: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_524,
        source_type: SourceType::Wikidata,
        name: "Install Maker project",
        extensions: &["iit"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x00, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
