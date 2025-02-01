use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857534: FileFormat = FileFormat {
    id: 105_857_534,
    puid: "wikidata/105857534",
    name: "HFS+ / Mac OS Extended disk image",
    extensions: &["hfs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x2B])],
            },
        }],
    }],
    related_formats: &[],
};
