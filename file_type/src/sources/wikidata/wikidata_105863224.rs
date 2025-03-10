use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_224,
        source_type: SourceType::Wikidata,
        name: "MED Synth sound",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x48, 0x00, 0xFF, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
