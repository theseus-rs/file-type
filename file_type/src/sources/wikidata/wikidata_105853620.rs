use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853620: FileFormat = FileFormat {
    id: 105_853_620,
    puid: "wikidata/105853620",
    name: "Microsoft ASP.NET Web User Control (UTF)",
    extensions: &["ascx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C, 0x25, 0x40, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
