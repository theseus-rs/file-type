use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851516: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_516,
        source_type: SourceType::Wikidata,
        name: "LaTeX 2e document (with rem)",
        extensions: &["tex"],
        media_types: &["application/x-tex"],
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
