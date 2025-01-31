use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120966739: FileFormat = FileFormat {
    id: 120_966_739,
    puid: "wikidata/120966739",
    name: "Microsoft Money 2001 data",
    extensions: &["mn9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
