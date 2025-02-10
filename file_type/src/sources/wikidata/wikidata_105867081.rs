use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867081: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_081,
        source_type: SourceType::Wikidata,
        name: "Faase Paint-by-Numbers puzzle format",
        extensions: &["nf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x69, 0x64, 0x74, 0x68, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
