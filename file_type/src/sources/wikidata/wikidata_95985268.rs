use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95985268: FileType = FileType {
    file_format: &FileFormat {
        id: 95_985_268,
        source_type: SourceType::Wikidata,
        name: "Affymetrix CEL file format",
        extensions: &["cel"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x43, 0x45, 0x4C, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
