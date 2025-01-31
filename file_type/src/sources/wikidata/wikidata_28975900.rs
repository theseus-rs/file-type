use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975900: FileFormat = FileFormat {
    id: 28_975_900,
    puid: "wikidata/28975900",
    name: "Control surface geometry file",
    extensions: &["csf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
