use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850952: FileFormat = FileFormat {
    id: 105_850_952,
    source_type: SourceType::Wikidata,
    name: "Windows 8-10 Desktop Theme (with CRLF)",
    extensions: &["theme"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
