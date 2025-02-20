use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859545: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_545,
        source_type: SourceType::Wikidata,
        name: "Verilog source code (with rem 1)",
        extensions: &["v"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
