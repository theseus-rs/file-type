use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_663,
        source_type: SourceType::Wikidata,
        name: "TTComp archive compressed (ASCII-1K)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
