use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857098: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_098,
        source_type: SourceType::Wikidata,
        name: "Storage card file segments Tiger Tree Hash",
        extensions: &["gltth"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x6C, 0x2B, 0x2B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
