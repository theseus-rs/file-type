use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850139: FileFormat = FileFormat {
    id: 105_850_139,
    puid: "wikidata/105850139",
    name: "Adobe Photoshop Custom Shape",
    extensions: &["csh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x75, 0x73, 0x68, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
