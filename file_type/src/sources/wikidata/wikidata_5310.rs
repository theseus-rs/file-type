use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5310: FileFormat = FileFormat {
    id: 5_310,
    source_type: SourceType::Wikidata,
    name: "LaTeX",
    extensions: &["tex"],
    media_types: &["application/x-latex"],
    signatures: &[],
    related_formats: &[],
};
