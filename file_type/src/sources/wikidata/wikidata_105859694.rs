use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859694: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_694,
        source_type: SourceType::Wikidata,
        name: "Xilinx instantiation template",
        extensions: &["vho"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
