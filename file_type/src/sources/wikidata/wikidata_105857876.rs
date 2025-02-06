use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857876: FileFormat = FileFormat {
    id: 105_857_876,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine user interface description (V1)",
    extensions: &["chu"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x48, 0x55, 0x49, 0x56, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
