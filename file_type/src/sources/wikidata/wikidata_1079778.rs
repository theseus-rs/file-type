use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1079778: FileFormat = FileFormat {
    id: 1_079_778,
    puid: "wikidata/1079778",
    name: "SIS",
    extensions: &["sis", "sisx"],
    media_types: &[
        "application/vnd.symbian.install",
        "application/vnd.symbian.install",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
