use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63098174: FileFormat = FileFormat {
    id: 63_098_174,
    puid: "wikidata/63098174",
    name: "Microsoft Visio XML Drawing, version 2003-2010",
    extensions: &["vdx"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
