use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206657: FileFormat = FileFormat {
    id: 28_206_657,
    puid: "wikidata/28206657",
    name: "Nero CoverDesigner Document",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
