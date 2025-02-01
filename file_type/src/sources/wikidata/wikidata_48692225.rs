use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48692225: FileFormat = FileFormat {
    id: 48_692_225,
    puid: "wikidata/48692225",
    name: "Microsoft Visio Drawing, version 2000",
    extensions: &["vsd"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
