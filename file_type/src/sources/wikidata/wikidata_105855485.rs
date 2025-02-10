use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855485: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_485,
        source_type: SourceType::Wikidata,
        name: "FidoCAD Library",
        extensions: &["fcl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x49, 0x44, 0x4F, 0x4C, 0x49, 0x42, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
