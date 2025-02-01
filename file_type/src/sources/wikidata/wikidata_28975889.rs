use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975889: FileFormat = FileFormat {
    id: 28_975_889,
    puid: "wikidata/28975889",
    name: "Potential Control File",
    extensions: &["pot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
