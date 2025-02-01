use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850059: FileFormat = FileFormat {
    id: 105_850_059,
    puid: "wikidata/105850059",
    name: "Windows Clipboard (BK)",
    extensions: &["clp"],
    media_types: &["application/windows-clipboard"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0xC3])],
            },
        }],
    }],
    related_formats: &[],
};
