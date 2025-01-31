use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837028: FileFormat = FileFormat {
    id: 108_837_028,
    puid: "wikidata/108837028",
    name: "Nero CD EXTRA Compilation",
    extensions: &["nrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
