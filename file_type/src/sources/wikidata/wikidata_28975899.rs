use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975899: FileFormat = FileFormat {
    id: 28_975_899,
    puid: "wikidata/28975899",
    name: "MultiSurf geometry file",
    extensions: &["ms2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
