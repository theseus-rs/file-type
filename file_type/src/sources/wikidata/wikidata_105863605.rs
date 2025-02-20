use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863605: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_605,
        source_type: SourceType::Wikidata,
        name: "Luigi's Mansion 3D model",
        extensions: &["mdl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0xB4, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
