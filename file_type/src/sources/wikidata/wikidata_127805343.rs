use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127805343: FileFormat = FileFormat {
    id: 127_805_343,
    puid: "wikidata/127805343",
    name: "njs script file",
    extensions: &["njs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
