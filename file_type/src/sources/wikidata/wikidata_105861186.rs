use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_186,
        source_type: SourceType::Wikidata,
        name: "X-Plane Painted Line",
        extensions: &["lin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
