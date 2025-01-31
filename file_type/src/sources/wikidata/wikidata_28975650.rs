use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975650: FileFormat = FileFormat {
    id: 28_975_650,
    puid: "wikidata/28975650",
    name: "Recon Mesh",
    extensions: &["m"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
