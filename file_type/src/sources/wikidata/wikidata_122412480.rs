use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122412480: FileFormat = FileFormat {
    id: 122_412_480,
    puid: "wikidata/122412480",
    name: "Merge File",
    extensions: &["mer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
