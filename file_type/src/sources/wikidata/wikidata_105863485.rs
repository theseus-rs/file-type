use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863485: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_485,
        source_type: SourceType::Wikidata,
        name: "Gmsh Mesh",
        extensions: &["msh"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x4D, 0x65, 0x73, 0x68, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
