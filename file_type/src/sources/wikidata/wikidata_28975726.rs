use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975726: FileFormat = FileFormat {
    id: 28_975_726,
    puid: "wikidata/28975726",
    name: "Gaussian Cube File",
    extensions: &["cub"],
    media_types: &["chemical/x-gaussian-cube"],
    internal_signatures: &[],
    related_formats: &[],
};
