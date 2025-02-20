use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_076,
        source_type: SourceType::Wikidata,
        name: "athtune script",
        extensions: &["athtune"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x73, 0x79, 0x6E, 0x74, 0x68, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
