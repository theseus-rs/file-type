use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207481: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_481,
        source_type: SourceType::Wikidata,
        name: "Webshots picture WBZ",
        extensions: &["wbz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0xAB, 0x91, 0x67])],
                },
            }],
        }],
        related_formats: &[],
    },
};
