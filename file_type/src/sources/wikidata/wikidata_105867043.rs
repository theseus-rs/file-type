use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867043: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_043,
        source_type: SourceType::Wikidata,
        name: "NeXus HDF4 data format",
        extensions: &["h4", "hdf", "hdf4", "nexus", "nx4", "nxs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0E, 0x03, 0x13, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
