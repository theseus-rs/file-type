use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110130210: FileFormat = FileFormat {
    id: 110_130_210,
    puid: "wikidata/110130210",
    name: "Microsoft Visio Drawing, version 4",
    extensions: &["vsd", "vss", "vst", "vsw"],
    media_types: &[
        "application/vnd.visio",
        "application/vnd.visio",
        "application/vnd.visio",
        "application/vnd.visio",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
