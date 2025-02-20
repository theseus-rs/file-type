use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850075: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_075,
        source_type: SourceType::Wikidata,
        name: "Kerbal Space Program (KSP) spacecraft",
        extensions: &["craft"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x68, 0x69, 0x70, 0x20, 0x3D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
