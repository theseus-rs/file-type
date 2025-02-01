use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96143857: FileFormat = FileFormat {
    id: 96_143_857,
    puid: "wikidata/96143857",
    name: "SurferGrid format",
    extensions: &["grd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
