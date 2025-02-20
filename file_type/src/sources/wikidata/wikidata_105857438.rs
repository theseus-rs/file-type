use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_438,
        source_type: SourceType::Wikidata,
        name: "Polyfilm 3D model",
        extensions: &["3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x6C, 0x79, 0x5F, 0x4F, 0x62, 0x6A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
