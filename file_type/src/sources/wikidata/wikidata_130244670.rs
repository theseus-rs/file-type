use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130244670: FileFormat = FileFormat {
    id: 130_244_670,
    puid: "wikidata/130244670",
    name: "LiveScript source code file",
    extensions: &["ls"],
    media_types: &["text/livescript"],
    internal_signatures: &[],
    related_formats: &[],
};
