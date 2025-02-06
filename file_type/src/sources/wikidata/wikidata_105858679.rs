use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858679: FileFormat = FileFormat {
    id: 105_858_679,
    source_type: SourceType::Wikidata,
    name: "ZX Spectrum CHR$ bitmap",
    extensions: &["ch$"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x68, 0x72, 0x24])],
            },
        }],
    }],
    related_formats: &[],
};
