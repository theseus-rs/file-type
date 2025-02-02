use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859274: FileFormat = FileFormat {
    id: 105_859_274,
    source_type: SourceType::Wikidata,
    name: "BibTeX",
    extensions: &["bib"],
    media_types: &["application/x-bibtex", "text/x-bibtex"],
    internal_signatures: &[],
    related_formats: &[],
};
