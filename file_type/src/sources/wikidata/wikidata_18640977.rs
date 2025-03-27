use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_18640977: FileType = FileType {
    file_format: &FileFormat {
        id: 18_640_977,
        source_type: SourceType::Wikidata,
        name: "Better Portable Graphics",
        extensions: &["bpg"],
        media_types: &["image/bpg", "image/x-bpg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x50, 0x47, 0xFB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
