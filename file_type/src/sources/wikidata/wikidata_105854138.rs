use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854138: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_138,
        source_type: SourceType::Wikidata,
        name: "Binary ][ archive",
        extensions: &["bny", "bqy"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x47, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
