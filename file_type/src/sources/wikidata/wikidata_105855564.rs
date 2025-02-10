use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855564: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_564,
        source_type: SourceType::Wikidata,
        name: "OMAX Make tool path data (with rem)",
        extensions: &["omx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
