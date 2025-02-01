use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116804318: FileFormat = FileFormat {
    id: 116_804_318,
    puid: "wikidata/116804318",
    name: "TimeWiz Catalog File",
    extensions: &["twc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
