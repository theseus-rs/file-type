use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975664: FileFormat = FileFormat {
    id: 28_975_664,
    source_type: SourceType::Wikidata,
    name: "AC3D",
    extensions: &["ac"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x33, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
