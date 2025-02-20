use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853194: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_194,
        source_type: SourceType::Wikidata,
        name: "Korg Song file",
        extensions: &["sng"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x4F, 0x52, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
