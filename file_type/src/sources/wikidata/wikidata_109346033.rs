use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109346033: FileFormat = FileFormat {
    id: 109_346_033,
    puid: "wikidata/109346033",
    name: "osu! beatmap archive (.osz)",
    extensions: &["osz"],
    media_types: &["application/x-osu-beatmap-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
