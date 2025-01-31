use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130542392: FileFormat = FileFormat {
    id: 130_542_392,
    puid: "wikidata/130542392",
    name: "Parallel Thread Execution file format",
    extensions: &["ptx"],
    media_types: &["text/x-ptx"],
    internal_signatures: &[],
    related_formats: &[],
};
