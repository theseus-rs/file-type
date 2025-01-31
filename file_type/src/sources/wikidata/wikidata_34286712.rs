use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34286712: FileFormat = FileFormat {
    id: 34_286_712,
    puid: "wikidata/34286712",
    name: "Processing Development Environment",
    extensions: &["pde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
