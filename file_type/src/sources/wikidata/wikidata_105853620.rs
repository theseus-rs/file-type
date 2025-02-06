use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853620: FileFormat = FileFormat {
    id: 105_853_620,
    source_type: SourceType::Wikidata,
    name: "Microsoft ASP.NET Web User Control (UTF)",
    extensions: &["ascx"],
    media_types: &[],
    signatures: &[Signature {
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
