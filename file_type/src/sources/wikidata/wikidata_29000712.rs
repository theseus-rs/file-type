use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29000712: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_712,
        source_type: SourceType::Wikidata,
        name: "TecPlot ASCII",
        extensions: &["tp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45])],
                },
            }],
        }],
        related_formats: &[],
    },
};
