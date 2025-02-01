use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858885: FileFormat = FileFormat {
    id: 105_858_885,
    puid: "wikidata/105858885",
    name: "GFA Raytrace Compressed image (hi-res) bitmap",
    extensions: &["sch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x63, 0x68, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
