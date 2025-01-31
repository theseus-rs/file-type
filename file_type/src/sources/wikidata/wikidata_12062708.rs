use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_12062708: FileFormat = FileFormat {
    id: 12_062_708,
    puid: "wikidata/12062708",
    name: "CDW file format",
    extensions: &["cdw"],
    media_types: &["image/cdw"],
    internal_signatures: &[],
    related_formats: &[],
};
