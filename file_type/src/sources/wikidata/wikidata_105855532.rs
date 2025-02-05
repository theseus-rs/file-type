use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855532: FileFormat = FileFormat {
    id: 105_855_532,
    source_type: SourceType::Wikidata,
    name: "Ots Media Library",
    extensions: &["omx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x4F, 0x74, 0x73, 0x4D, 0x58, 0x21, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
