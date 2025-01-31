use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857207: FileFormat = FileFormat {
    id: 105_857_207,
    puid: "wikidata/105857207",
    name: "OPHelp Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x50, 0x48, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
