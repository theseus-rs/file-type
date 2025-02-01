use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757740: FileFormat = FileFormat {
    id: 28_757_740,
    puid: "wikidata/28757740",
    name: "GEM VDI Metafile",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
