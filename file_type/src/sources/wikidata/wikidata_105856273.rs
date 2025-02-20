use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856273: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_273,
        source_type: SourceType::Wikidata,
        name: "Virtuoso Display Resource File (with rem)",
        extensions: &["drf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
