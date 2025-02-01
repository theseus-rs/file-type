use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111511881: FileFormat = FileFormat {
    id: 111_511_881,
    puid: "wikidata/111511881",
    name: "ESRI ArcInfo Coverage Annotation File",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
