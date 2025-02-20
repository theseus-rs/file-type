use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_739,
        source_type: SourceType::Wikidata,
        name: "Mod2PSG2 PSGMOD module (v5)",
        extensions: &["psgmod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x47, 0x4D, 0x4F, 0x44, 0x20, 0x20, 0x46, 0x4F, 0x52, 0x4D,
                        0x41, 0x54, 0x20, 0x35,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
