use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_283579: FileType = FileType {
    file_format: &FileFormat {
        id: 283_579,
        source_type: SourceType::Wikidata,
        name: "tar",
        extensions: &["tar"],
        media_types: &["application/x-tar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x75, 0x73, 0x74, 0x61, 0x72])],
                },
            }],
        }],
        related_formats: &[],
    },
};
