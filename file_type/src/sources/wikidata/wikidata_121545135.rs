use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121545135: FileFormat = FileFormat {
    id: 121_545_135,
    puid: "wikidata/121545135",
    name: "At Home 2012 Tax Return File",
    extensions: &["t12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
