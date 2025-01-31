use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777707: FileFormat = FileFormat {
    id: 28_777_707,
    puid: "wikidata/28777707",
    name: "mzML",
    extensions: &["mxml", "mzML", "mzml"],
    media_types: &["text/xml", "text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
