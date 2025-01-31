use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865369: FileFormat = FileFormat {
    id: 105_865_369,
    puid: "wikidata/105865369",
    name: "PatchMeister Driver",
    extensions: &["pmdriver"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x44, 0x52, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
