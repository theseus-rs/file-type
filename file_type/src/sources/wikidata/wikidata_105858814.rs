use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_814,
        source_type: SourceType::Wikidata,
        name: "CUPS Raster bitmap (v2, BE)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x61, 0x53, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
