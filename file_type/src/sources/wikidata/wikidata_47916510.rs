use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47916510: FileFormat = FileFormat {
    id: 47_916_510,
    puid: "wikidata/47916510",
    name: "Microsoft Excel Web Query",
    extensions: &["iqy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x45, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
