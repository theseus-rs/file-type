use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858614: FileFormat = FileFormat {
    id: 105_858_614,
    source_type: SourceType::Wikidata,
    name: "ImageMagick Machine independent File Format bitmap (with rem)",
    extensions: &["mif", "miff"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
