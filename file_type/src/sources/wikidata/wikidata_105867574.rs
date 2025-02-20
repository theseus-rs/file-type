use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867574: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_574,
        source_type: SourceType::Wikidata,
        name: "NAPLPS graphics",
        extensions: &["nap"],
        media_types: &["image/naplps"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x18, 0x1B, 0x22, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
