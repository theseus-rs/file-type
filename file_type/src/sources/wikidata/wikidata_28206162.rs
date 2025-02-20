use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206162: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_162,
        source_type: SourceType::Wikidata,
        name: "GEM Raster",
        extensions: &["img"],
        media_types: &["application/gem"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEB, 0x3C, 0x90, 0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
