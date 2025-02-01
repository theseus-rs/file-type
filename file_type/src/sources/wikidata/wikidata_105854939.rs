use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854939: FileFormat = FileFormat {
    id: 105_854_939,
    puid: "wikidata/105854939",
    name: "ENhanced Compressor compressed archive",
    extensions: &["enc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x6E, 0x63, 0x68, 0x00, 0x0F, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
