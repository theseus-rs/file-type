use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855276: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_276,
        source_type: SourceType::Wikidata,
        name: "Fasoo DRM encrypted",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x9B, 0x20, 0x44, 0x52, 0x4D, 0x4F, 0x4E, 0x45, 0x20, 0x20, 0x54, 0x68,
                        0x69, 0x73, 0x20, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20,
                        0x69, 0x73, 0x20, 0x65, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64,
                        0x20, 0x61, 0x6E, 0x64, 0x20, 0x70, 0x72, 0x6F, 0x74, 0x65, 0x63, 0x74,
                        0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x46, 0x61, 0x73, 0x6F, 0x6F, 0x20,
                        0x44, 0x52, 0x4D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
