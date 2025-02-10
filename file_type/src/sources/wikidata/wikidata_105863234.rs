use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863234: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_234,
        source_type: SourceType::Wikidata,
        name: "Magnetic Sound",
        extensions: &["snd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x61, 0x53, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
