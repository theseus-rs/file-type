use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110098687: FileFormat = FileFormat {
    id: 110_098_687,
    puid: "wikidata/110098687",
    name: "Microsoft Visio Drawing, version 2",
    extensions: &["vsd", "vss", "vst"],
    media_types: &[
        "application/vnd.visio",
        "application/vnd.visio",
        "application/vnd.visio",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
