use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206229: FileFormat = FileFormat {
    id: 28_206_229,
    puid: "wikidata/28206229",
    name: "Gridded Binary",
    extensions: &["grb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
