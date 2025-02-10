use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28770330: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_330,
        source_type: SourceType::Wikidata,
        name: "LightWave Scene",
        extensions: &["lws", "scn"],
        media_types: &["image/x-lws"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x53, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
