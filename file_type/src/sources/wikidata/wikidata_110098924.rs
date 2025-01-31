use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110098924: FileFormat = FileFormat {
    id: 110_098_924,
    puid: "wikidata/110098924",
    name: "Microsoft Visio Drawing, version 3",
    extensions: &["vsd", "vss", "vst"],
    media_types: &[
        "application/vnd.visio",
        "application/vnd.visio",
        "application/vnd.visio",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
