use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3547199: FileFormat = FileFormat {
    id: 3_547_199,
    puid: "wikidata/3547199",
    name: "UHARC archive",
    extensions: &["uha"],
    media_types: &["application/x-uha-compressed"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x48, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
