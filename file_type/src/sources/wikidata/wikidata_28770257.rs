use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770257: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_257,
        source_type: SourceType::Wikidata,
        name: "HydroCAD Stormwater Modeling System Unit Hydrograph file",
        extensions: &["huh"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2F, 0x2F, 0x20, 0x48, 0x79, 0x64, 0x72, 0x6F, 0x43, 0x41, 0x44, 0x20,
                        0x55, 0x6E, 0x69, 0x74, 0x20, 0x48, 0x79, 0x64, 0x72, 0x6F, 0x67, 0x72,
                        0x61, 0x70, 0x68, 0x20, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
