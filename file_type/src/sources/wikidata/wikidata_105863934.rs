use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863934: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_934,
        source_type: SourceType::Wikidata,
        name: "MetaImage MetaHeader",
        extensions: &["mhd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x20, 0x3D,
                        0x20, 0x49, 0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
