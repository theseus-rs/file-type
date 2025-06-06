use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856333: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_333,
        source_type: SourceType::Wikidata,
        name: "Lotus Manuscript Document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x52, 0x32, 0x0D, 0x0A, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
