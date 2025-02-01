use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344985: FileFormat = FileFormat {
    id: 28_344_985,
    puid: "wikidata/28344985",
    name: "Genital Save State",
    extensions: &["gns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
