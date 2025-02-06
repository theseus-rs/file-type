use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858232: FileFormat = FileFormat {
    id: 105_858_232,
    source_type: SourceType::Wikidata,
    name: "EditPad Pro Project",
    extensions: &["epp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
