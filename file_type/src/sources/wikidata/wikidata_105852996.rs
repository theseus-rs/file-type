use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852996: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_996,
        source_type: SourceType::Wikidata,
        name: "Caligari TrueSpace 3D object (v2.x)",
        extensions: &["sobj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x4C, 0x42, 0x53, 0x4F, 0x42, 0x4A, 0x56, 0x45, 0x52, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
