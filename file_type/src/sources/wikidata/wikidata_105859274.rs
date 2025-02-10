use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859274: FileFormat = FileFormat {
    id: 105_859_274,
    source_type: SourceType::Wikidata,
    name: "BibTeX",
    extensions: &["bib"],
    media_types: &["application/x-bibtex", "text/x-bibtex"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40])],
            },
        }],
    }],
    related_formats: &[],
};
