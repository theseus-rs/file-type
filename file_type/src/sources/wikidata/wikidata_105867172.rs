use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867172: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_172,
        source_type: SourceType::Wikidata,
        name: "NeXus HDF5 data format",
        extensions: &["h5", "hdf", "nexus", "nx5", "nxs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x89, 0x48, 0x44, 0x46, 0x0D, 0x0A, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
