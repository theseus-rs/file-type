use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130358240: FileFormat = FileFormat {
    id: 130_358_240,
    puid: "wikidata/130358240",
    name: "Mscgen file format",
    extensions: &["msc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
