use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131299780: FileFormat = FileFormat {
    id: 131_299_780,
    puid: "wikidata/131299780",
    name: "ThingsDB file format",
    extensions: &["ti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
