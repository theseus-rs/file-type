use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857592: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_592,
        source_type: SourceType::Wikidata,
        name: "Generic IFF container",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4F, 0x52, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
