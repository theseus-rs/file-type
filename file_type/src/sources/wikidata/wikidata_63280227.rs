use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63280227: FileFormat = FileFormat {
    id: 63_280_227,
    puid: "wikidata/63280227",
    name: "Microsoft Works Database for Windows, version 4.0a",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
