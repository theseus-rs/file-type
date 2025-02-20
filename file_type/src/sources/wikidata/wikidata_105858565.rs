use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858565: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_565,
        source_type: SourceType::Wikidata,
        name: "Ipix Spherical Panorama",
        extensions: &["ipx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x62])],
                },
            }],
        }],
        related_formats: &[],
    },
};
