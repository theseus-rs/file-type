use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860102: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_102,
        source_type: SourceType::Wikidata,
        name: "Verilog source code",
        extensions: &["v"],
        media_types: &["text/x-verilog"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
