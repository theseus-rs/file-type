use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851516: FileFormat = FileFormat {
    id: 105_851_516,
    puid: "wikidata/105851516",
    name: "LaTeX 2e document (with rem)",
    extensions: &["tex"],
    media_types: &["application/x-tex"],
    internal_signatures: &[],
    related_formats: &[],
};
