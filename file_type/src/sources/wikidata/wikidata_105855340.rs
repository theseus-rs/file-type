use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855340: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_340,
        source_type: SourceType::Wikidata,
        name: "PPrint stencil Format",
        extensions: &["frm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x46, 0x4F, 0x52, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
