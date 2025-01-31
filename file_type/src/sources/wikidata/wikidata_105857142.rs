use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857142: FileFormat = FileFormat {
    id: 105_857_142,
    puid: "wikidata/105857142",
    name: "VGAPaint 386 Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x48, 0x4C, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
