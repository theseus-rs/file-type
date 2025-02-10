use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867302: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_302,
        source_type: SourceType::Wikidata,
        name: "Channel Data File",
        extensions: &["ndf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x44, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
