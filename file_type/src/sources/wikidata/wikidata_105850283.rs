use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850283: FileFormat = FileFormat {
    id: 105_850_283,
    puid: "wikidata/105850283",
    name: "TreeSheets project",
    extensions: &["cts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x53, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
