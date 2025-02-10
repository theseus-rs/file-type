use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853084: FileFormat = FileFormat {
    id: 105_853_084,
    source_type: SourceType::Wikidata,
    name: "SquashSF image file (little endian)",
    extensions: &["sfs", "squashfs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x68, 0x73, 0x71, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
