use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10852293: FileFormat = FileFormat {
    id: 10_852_293,
    puid: "wikidata/10852293",
    name: "RPT",
    extensions: &["rpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
