use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866058: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_058,
        source_type: SourceType::Wikidata,
        name: "ProtoBuf binary format map data",
        extensions: &["pbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x0D, 0x0A, 0x09, 0x4F, 0x53, 0x4D, 0x48, 0x65, 0x61,
                        0x64, 0x65, 0x72, 0x18,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
