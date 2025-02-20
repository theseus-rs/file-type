use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205771: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_771,
        source_type: SourceType::Wikidata,
        name: "BigTIFF",
        extensions: &["btf", "tf8", "tif", "tiff"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4D, 0x4D, 0x00, 0x2B])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x49, 0x49, 0x2B, 0x00])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
