use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_5533904: FileType = FileType {
    file_format: &FileFormat {
        id: 5_533_904,
        source_type: SourceType::Wikidata,
        name: "GeoJSON",
        extensions: &["geojson"],
        media_types: &["application/geo+json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
