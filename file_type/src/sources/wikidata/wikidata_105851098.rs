use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_098,
        source_type: SourceType::Wikidata,
        name: "LaTeX 2e document",
        extensions: &["tex"],
        media_types: &["application/x-tex"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x63, 0x6C, 0x61,
                        0x73, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
