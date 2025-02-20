use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850813: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_813,
        source_type: SourceType::Wikidata,
        name: "ED editor Keys definitions",
        extensions: &["key"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x65, 0x79, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
