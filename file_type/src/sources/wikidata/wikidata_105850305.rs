use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850305: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_305,
        source_type: SourceType::Wikidata,
        name: "SeeYou Raster Map",
        extensions: &["cmr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x55, 0x4D, 0x41, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
