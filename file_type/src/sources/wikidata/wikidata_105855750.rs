use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855750: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_750,
        source_type: SourceType::Wikidata,
        name: "Bentley MicroStation CAD drawing (simple)",
        extensions: &["dgn"],
        media_types: &["application/x-bentley-dgn"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0x09, 0xFE, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
