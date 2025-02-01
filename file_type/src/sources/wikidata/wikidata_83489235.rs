use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83489235: FileFormat = FileFormat {
    id: 83_489_235,
    puid: "wikidata/83489235",
    name: "VisiCalc file format",
    extensions: &["vc", "vcs"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
