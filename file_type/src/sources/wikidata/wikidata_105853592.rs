use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853592: FileFormat = FileFormat {
    id: 105_853_592,
    source_type: SourceType::Wikidata,
    name: "ZAP Program description",
    extensions: &["zap"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x72, 0x6F, 0x67, 0x6E, 0x61, 0x6D, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
