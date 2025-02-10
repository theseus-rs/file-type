use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857404: FileFormat = FileFormat {
    id: 105_857_404,
    source_type: SourceType::Wikidata,
    name: "DroidPad custom layout",
    extensions: &["json"],
    media_types: &["application/json"],
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
