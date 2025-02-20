use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863894: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_894,
        source_type: SourceType::Wikidata,
        name: "MSX MoonDriver song",
        extensions: &["mdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x52, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
