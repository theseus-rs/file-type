use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_267,
        source_type: SourceType::Wikidata,
        name: "GNU Bison grammar (with rem)",
        extensions: &["yy"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
