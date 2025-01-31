use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34273453: FileFormat = FileFormat {
    id: 34_273_453,
    puid: "wikidata/34273453",
    name: "Keynote Zipped",
    extensions: &["key.zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
