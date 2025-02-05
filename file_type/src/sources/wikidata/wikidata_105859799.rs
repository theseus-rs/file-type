use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859799: FileFormat = FileFormat {
    id: 105_859_799,
    source_type: SourceType::Wikidata,
    name: "Icarus Verilog VVP format (with Shebang)",
    extensions: &["vvp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
