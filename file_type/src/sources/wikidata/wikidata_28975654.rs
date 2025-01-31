use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975654: FileFormat = FileFormat {
    id: 28_975_654,
    puid: "wikidata/28975654",
    name: "Recon Points",
    extensions: &["pts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
