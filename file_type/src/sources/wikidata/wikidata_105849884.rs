use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_884,
        source_type: SourceType::Wikidata,
        name: "SRI PeakSimple chromatogram",
        extensions: &["chr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x54, 0x59, 0x50, 0x45, 0x3E, 0x3D, 0x43, 0x48, 0x52, 0x4F, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
