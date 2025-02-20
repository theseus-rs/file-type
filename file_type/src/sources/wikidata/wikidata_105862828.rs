use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862828: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_828,
        source_type: SourceType::Wikidata,
        name: "Mass Effect 3 Head Morph",
        extensions: &["me3headmorph"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x49, 0x42, 0x42, 0x45, 0x44, 0x4D, 0x41, 0x53, 0x53, 0x45, 0x46,
                        0x46, 0x45, 0x43, 0x54, 0x33, 0x48, 0x45, 0x41, 0x44, 0x4D, 0x4F, 0x52,
                        0x50, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
