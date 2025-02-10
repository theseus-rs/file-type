use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_209,
        source_type: SourceType::Wikidata,
        name: "MetaQuotes Language 5 compiled program",
        extensions: &["ex4"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x58, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
