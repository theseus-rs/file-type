use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856889: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_889,
        source_type: SourceType::Wikidata,
        name: "Game Description Language",
        extensions: &["gdl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x72, 0x6F, 0x6C, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
