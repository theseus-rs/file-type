use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66663925: FileFormat = FileFormat {
    id: 66_663_925,
    puid: "wikidata/66663925",
    name: "OS/2 Metafile",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
