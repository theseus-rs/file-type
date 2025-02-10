use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857670: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_670,
        source_type: SourceType::Wikidata,
        name: "Citrix Independent Computer Architecture",
        extensions: &["ica"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
