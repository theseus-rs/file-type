use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858397: FileFormat = FileFormat {
    id: 105_858_397,
    source_type: SourceType::Wikidata,
    name: "SimCity 4 Exemplar (text)",
    extensions: &["exmp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x51, 0x5A, 0x54, 0x31, 0x23, 0x23, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
