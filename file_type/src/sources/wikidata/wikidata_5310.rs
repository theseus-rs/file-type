use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5310: FileFormat = FileFormat {
    id: 5_310,
    source_type: SourceType::Wikidata,
    name: "LaTeX",
    extensions: &["tex"],
    media_types: &["application/x-latex"],
    internal_signatures: &[],
    related_formats: &[],
};
