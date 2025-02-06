use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853024: FileFormat = FileFormat {
    id: 105_853_024,
    source_type: SourceType::Wikidata,
    name: "PHAST Sufficient Statistics",
    extensions: &["ss"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x53, 0x45, 0x51, 0x53, 0x20, 0x3D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
