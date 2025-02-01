use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850952: FileFormat = FileFormat {
    id: 105_850_952,
    puid: "wikidata/105850952",
    name: "Windows 8-10 Desktop Theme (with CRLF)",
    extensions: &["theme"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
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
