use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851182: FileFormat = FileFormat {
    id: 105_851_182,
    puid: "wikidata/105851182",
    name: "LaTeX table of contents",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
