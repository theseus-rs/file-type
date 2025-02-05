use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855701: FileFormat = FileFormat {
    id: 105_855_701,
    source_type: SourceType::Wikidata,
    name: "OrgPlus Organization Chart",
    extensions: &["opx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4F, 0x43, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
