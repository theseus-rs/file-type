use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852411: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_411,
        source_type: SourceType::Wikidata,
        name: "Swish-e index",
        extensions: &["swish"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x53, 0x57, 0x49, 0x53, 0x48, 0x20, 0x66, 0x6F, 0x72, 0x6D,
                        0x61, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
