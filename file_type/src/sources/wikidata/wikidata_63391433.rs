use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63391433: FileFormat = FileFormat {
    id: 63_391_433,
    puid: "wikidata/63391433",
    name: "Microsoft Works Database for DOS",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
