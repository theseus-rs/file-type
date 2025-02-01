use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857568: FileFormat = FileFormat {
    id: 105_857_568,
    puid: "wikidata/105857568",
    name: "blueMSX machine settings",
    extensions: &["ini"],
    media_types: &["text/ini"],
    internal_signatures: &[],
    related_formats: &[],
};
