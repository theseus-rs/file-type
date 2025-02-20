use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858254: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_254,
        source_type: SourceType::Wikidata,
        name: "FrontPage Theme-Pack",
        extensions: &["elm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x2E, 0x30, 0x2E, 0x32, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
