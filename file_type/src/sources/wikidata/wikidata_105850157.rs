use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850157: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_157,
        source_type: SourceType::Wikidata,
        name: "FGT virus infected 16-bit COM executable",
        extensions: &["com", "img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x51, 0x52, 0x56, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
