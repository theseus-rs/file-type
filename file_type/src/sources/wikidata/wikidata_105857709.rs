use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857709: FileFormat = FileFormat {
    id: 105_857_709,
    source_type: SourceType::Wikidata,
    name: "Image Spectrumizer Workspace (v4.0)",
    extensions: &["isw"],
    media_types: &["text/json"],
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
