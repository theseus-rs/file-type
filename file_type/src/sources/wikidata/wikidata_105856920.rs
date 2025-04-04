use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856920: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_920,
        source_type: SourceType::Wikidata,
        name: "GOM 3D data",
        extensions: &["g3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x47, 0x4F, 0x4D, 0x2D, 0x33, 0x44, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
