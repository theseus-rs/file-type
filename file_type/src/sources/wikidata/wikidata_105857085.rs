use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857085: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_085,
        source_type: SourceType::Wikidata,
        name: "Granite Devices Firmware (v310)",
        extensions: &["gdf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x44, 0x46, 0x57, 0x36, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
