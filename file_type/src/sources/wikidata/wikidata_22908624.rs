use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_22908624: FileType = FileType {
    file_format: &FileFormat {
        id: 22_908_624,
        source_type: SourceType::Wikidata,
        name: "GeoPackage",
        extensions: &["gpkg"],
        media_types: &["application/geopackage+sqlite3", "application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x33,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
