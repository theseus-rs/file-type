use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849272: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_272,
        source_type: SourceType::Wikidata,
        name: "GNU Bison grammar",
        extensions: &["yy"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
