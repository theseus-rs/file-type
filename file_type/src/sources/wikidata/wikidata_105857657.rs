use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857657: FileFormat = FileFormat {
    id: 105_857_657,
    puid: "wikidata/105857657",
    name: "Adobe InDesign Markup Language",
    extensions: &["idml"],
    media_types: &["application/vnd.adobe.indesign-idml-package"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
