use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1437034: FileFormat = FileFormat {
    id: 1_437_034,
    puid: "wikidata/1437034",
    name: "Photoshop Big",
    extensions: &["psb"],
    media_types: &["image/vnd.adobe.photoshop"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x42, 0x50, 0x53, 0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
