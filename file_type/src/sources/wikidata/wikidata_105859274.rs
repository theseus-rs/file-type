use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859274: FileFormat = FileFormat {
    id: 105_859_274,
    puid: "wikidata/105859274",
    name: "BibTeX",
    extensions: &["bib", "bib"],
    media_types: &["application/x-bibtex", "text/x-bibtex"],
    internal_signatures: &[],
    related_formats: &[],
};
