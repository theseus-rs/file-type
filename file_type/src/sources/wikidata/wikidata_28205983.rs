use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205983: FileFormat = FileFormat {
    id: 28_205_983,
    puid: "wikidata/28205983",
    name: "Radiance Scene Description",
    extensions: &["rad"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
