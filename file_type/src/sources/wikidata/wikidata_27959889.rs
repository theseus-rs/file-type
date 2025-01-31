use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959889: FileFormat = FileFormat {
    id: 27_959_889,
    puid: "wikidata/27959889",
    name: "Cubase arrangement",
    extensions: &["arr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
