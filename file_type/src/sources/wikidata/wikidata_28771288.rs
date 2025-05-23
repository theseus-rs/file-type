use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771288: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_288,
        source_type: SourceType::Wikidata,
        name: "Mapsforge Binary Map",
        extensions: &["map"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x61, 0x70, 0x73, 0x66, 0x6F, 0x72, 0x67, 0x65, 0x20, 0x62, 0x69,
                        0x6E, 0x61, 0x72, 0x79, 0x20, 0x4F, 0x53, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
