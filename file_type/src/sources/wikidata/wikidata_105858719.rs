use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858719: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_719,
        source_type: SourceType::Wikidata,
        name: "OME-TIFF bitmap",
        extensions: &["tif", "tiff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
