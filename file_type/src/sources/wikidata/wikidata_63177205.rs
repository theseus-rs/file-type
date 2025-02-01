use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63177205: FileFormat = FileFormat {
    id: 63_177_205,
    puid: "wikidata/63177205",
    name: "Microsoft Works Database for Macintosh, version 4",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
