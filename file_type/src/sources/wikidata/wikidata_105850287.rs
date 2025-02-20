use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_287,
        source_type: SourceType::Wikidata,
        name: "CryEngine Project (v5)",
        extensions: &["cryproject"],
        media_types: &["application/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
