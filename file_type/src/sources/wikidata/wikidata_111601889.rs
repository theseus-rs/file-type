use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111601889: FileFormat = FileFormat {
    id: 111_601_889,
    puid: "wikidata/111601889",
    name: "Adobe InDesign Document, version CC 2019",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
