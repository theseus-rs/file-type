use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863805: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_805,
        source_type: SourceType::Wikidata,
        name: "PS2 PowerSave",
        extensions: &["max"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x73, 0x32, 0x50, 0x6F, 0x77, 0x65, 0x72, 0x53, 0x61, 0x76, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
