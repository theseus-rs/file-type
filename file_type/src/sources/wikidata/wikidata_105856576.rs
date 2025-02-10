use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856576: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_576,
        source_type: SourceType::Wikidata,
        name: "The Witcher 2 Entity",
        extensions: &["w2ent"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x52, 0x32, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
