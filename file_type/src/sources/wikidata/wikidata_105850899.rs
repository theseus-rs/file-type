use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850899: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_899,
        source_type: SourceType::Wikidata,
        name: "Kundo smart card exchange Format",
        extensions: &["ktf"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
