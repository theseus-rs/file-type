use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99184084: FileType = FileType {
    file_format: &FileFormat {
        id: 99_184_084,
        source_type: SourceType::Wikidata,
        name: "Atom web feed",
        extensions: &["atom", "xml"],
        media_types: &["application/atom+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
